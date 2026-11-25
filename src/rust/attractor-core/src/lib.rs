#![warn(missing_docs)]
// pub mod attractor;

//! # attractor crate
//!
//! Create and Plot Strange Attractors
//!
//! Create and plot attractors, specifically an attractor described by Julian
//! Sprott in his book "_Strange Attractors: Creating Patterns in Chaos_".
//! <https://sprott.physics.wisc.edu/fractals/booktext/sabook.pdf>
//!
//! The resulting attractor is discretized (binned) into matrix, making plotting fast.
//!

use colorgrad::{Color, Gradient};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{ColorType, ImageBuffer, ImageEncoder, RgbaImage};
use inc_stats::Percentiles;
use ndarray::prelude::*;
use ndarray::Array1;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

/// Wall-clock time spent in each stage of rendering.
///
/// Stages not reached (for example colouring after a diverging orbit) stay zero.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    /// generating the orbit
    pub orbit: Duration,
    /// computing the percentile / quantile range
    pub range: Duration,
    /// binning points into the count matrix
    pub discretise: Duration,
    /// log-scaling the counts and finding their range
    pub rescale: Duration,
    /// mapping counts through the gradient into pixels
    pub colour: Duration,
    /// encoding and writing the PNG
    pub encode: Duration,
}

impl Timings {
    /// Sum of all stages
    pub fn total(&self) -> Duration {
        self.orbit + self.range + self.discretise + self.rescale + self.colour + self.encode
    }

    /// Stage names with their durations, in pipeline order
    pub fn stages(&self) -> [(&'static str, Duration); 6] {
        [
            ("orbit", self.orbit),
            ("range", self.range),
            ("discretise", self.discretise),
            ("rescale", self.rescale),
            ("colour", self.colour),
            ("encode", self.encode),
        ]
    }
}

/// Holds attractor results.
///
pub struct AttractorResult {
    /// filename
    pub filename: String,
    /// number of points computed
    pub n: usize,
    /// does the attractor remain in finite bounds
    pub finite: bool,
    /// time spent in each stage
    pub timings: Timings,
    /// why nothing was drawn, when that is not plain divergence
    pub error: Option<String>,
}

/// Attractor input configuration
///
#[derive(Debug)]
pub struct Attractor {
    /// coefficient array
    pub a: Array1<f64>,
    /// number of iterations
    pub n: usize,
    /// starting x and y value,
    pub filename: String,
    /// initial values
    pub xy: (f64, f64),
    /// image size
    pub img_size: (usize, usize),
    /// percentile threshold to limit sampling points
    pub perc_threshold: f64,
    /// color gradient
    pub gradient: Gradient,
    /// Generate the orbit as this many independent trajectories in parallel
    /// instead of one serial one. `None` (or `Some(1)`) is the serial orbit.
    ///
    /// The map is chaotic and its orbit converges onto the attractor, so
    /// trajectories from nearby starts draw the same picture, but the counts
    /// differ from the serial orbit in detail. The result is
    /// deterministic for a given count. The number is clamped so that each
    /// trajectory has at least 10,000 points (see [`Attractor::check`]).
    pub parallel_orbit: Option<usize>,
}

/// Scales (interpolates) value and maps onto a different output range
fn remap(x: &f64, from: (f64, f64), to: (f64, f64)) -> f64 {
    if from.0 == from.1 {
        return 0.5;
    }
    to.0 + (x - from.0) * (to.1 - to.0) / (from.1 - from.0)
}

/// Natural logarithm; from the pure-Rust `libm` crate with the `libm` feature.
///
/// The C runtime's `ln` and `exp` are several times slower under the
/// `x86_64-pc-windows-gnu` target that R packages use than under MSVC.
#[inline]
fn ln(x: f64) -> f64 {
    #[cfg(feature = "libm")]
    {
        libm::log(x)
    }
    #[cfg(not(feature = "libm"))]
    {
        x.ln()
    }
}

/// Exponential; see [`ln`].
#[inline]
fn exp(x: f64) -> f64 {
    #[cfg(feature = "libm")]
    {
        libm::exp(x)
    }
    #[cfg(not(feature = "libm"))]
    {
        x.exp()
    }
}

/// Generate strange attractor using equation 7E of Sprott.
///
/// This attractor is based on Equation 7E of Sprott (see references).
///
/// $ x_{i+1} = a_{1} + a_{2} * x_{i} +  a_{3} * y_{i} +  a_{4} * |x_{i}|^{a5}  +  a6 * |y_{i}|^{a7} $
///
/// \deqn{y_{i+1} = a_{8} + a_{9} * x_{i} +  a_{10} * y_{i} +  a_{11} * |x_{i}|^{a12}  +  a13 * |y_{i}|^{a14}}
///
/// # references
///
/// Julien C. Sprott, "Strange Attractors: Creating Patterns in Chaos", page 418, Equation 7e,
/// <https://sprott.physics.wisc.edu/fractals/booktext/sabook.pdf>
///
fn sprott_7e(a: &Array1<f64>, &x: &f64, &y: &f64) -> (f64, f64) {
    let a1 = a[0];
    let a2 = a[1];
    let a3 = a[2];
    let a4 = a[3];
    let a5 = a[4];
    let a6 = a[5];
    let a7 = a[6];
    let a8 = a[7];
    let a9 = a[8];
    let a10 = a[9];
    let a11 = a[10];
    let a12 = a[11];
    let a13 = a[12];
    let a14 = a[13];

    // |x|^p as exp(p ln|x|), so the two logarithms are shared by both equations:
    // two `ln` and four `exp` instead of four `powf`, about a third faster.
    // At x = 0 the power is exp(-inf) = 0, as for `powf` with a positive exponent.
    // An exponent of exactly 0 gives NaN where `powf` gives 1, which callers treat
    // as divergence (`is_finite`); it cannot arise from normally distributed
    // coefficients.
    let lx = ln(x.abs());
    let ly = ln(y.abs());
    let x1: f64 = a1 + a2 * x + a3 * y + a4 * exp(a5 * lx) + a6 * exp(a7 * ly);
    let y1: f64 = a8 + a9 * x + a10 * y + a11 * exp(a12 * lx) + a13 * exp(a14 * ly);

    (x1, y1)
}

impl Default for Attractor {
    /// Create a new attractor with default values
    fn default() -> Attractor {
        let sprott_default_seed: Array1<f64> = array!(
            -0.589143526796629,
            0.674281478192589,
            0.738887963849125,
            -0.303279513858981,
            0.777634792527011,
            0.0712083741377903,
            0.0940309871410827,
            -1.17350599974869,
            -0.780243673606325,
            0.536210512392793,
            1.60723417066195,
            -1.2742777716953,
            -0.0660871384554965,
            0.448090598863179
        );
        Attractor {
            a: sprott_default_seed,
            n: 100_000,
            filename: String::from("attractor.png"),
            xy: (1.0, 1.0),
            img_size: (1024, 768),
            perc_threshold: 0.95,
            gradient: colorgrad::viridis(),
            parallel_orbit: None,
        }
    }
}

impl Attractor {
    /// Computes the attractor and saves it as a PNG
    pub fn compute(&self) -> AttractorResult {
        if let Err(msg) = self.check() {
            return AttractorResult {
                filename: String::new(),
                n: 0,
                finite: false,
                timings: Timings::default(),
                error: Some(msg),
            };
        }
        let d = match self.default_counts() {
            Ok(d) => d,
            Err(msg) => {
                return AttractorResult {
                    filename: String::new(),
                    n: 0,
                    finite: false,
                    timings: Timings::default(),
                    error: Some(msg),
                };
            }
        };
        let mut timings = d.timings;
        if !d.finite {
            return AttractorResult {
                filename: String::from(""),
                n: d.n,
                finite: false,
                timings,
                error: None,
            };
        }

        // convert array to image with colour gradient
        let imgbuf: RgbaImage = convert_to_image(d.counts, &self.gradient, &mut timings);

        // save png image
        match save_attractor(imgbuf, &self.filename) {
            Ok(t) => timings.encode = t,
            Err(msg) => {
                return AttractorResult {
                    filename: String::new(),
                    n: d.n,
                    finite: true,
                    timings,
                    error: Some(msg),
                };
            }
        }

        AttractorResult {
            filename: self.filename.to_string(),
            n: d.n,
            finite: true,
            timings,
            error: None,
        }
    }

    /// Orbit, range and binning only: the count matrix, without colouring or a file.
    ///
    /// Returns `Err` for an invalid configuration or an orbit that collapsed.
    /// A diverging orbit is `Ok` with `finite == false`.
    pub fn counts(&self) -> Result<Counts, String> {
        self.check()?;
        self.default_counts()
    }

    /// The raw orbit: `n` points after a 1,000-point burn-in, from the serial orbit.
    ///
    /// Returns `None` if the orbit diverges (a non-finite value appears).
    pub fn orbit(&self) -> Option<Vec<(f64, f64)>> {
        let (mut x, mut y) = self.xy;
        for _ in 0..1000 {
            (x, y) = sprott_7e(&self.a, &x, &y);
            if !x.is_finite() || !y.is_finite() {
                return None;
            }
        }
        let mut out = Vec::with_capacity(self.n);
        for _ in 0..self.n {
            (x, y) = sprott_7e(&self.a, &x, &y);
            if !x.is_finite() || !y.is_finite() {
                return None;
            }
            out.push((x, y));
        }
        Some(out)
    }

    /// Check the configuration, returning a message for a setting that cannot work.
    pub fn check(&self) -> Result<(), String> {
        if self.parallel_orbit == Some(0) {
            return Err("parallel orbit needs at least 1 trajectory".to_string());
        }
        Ok(())
    }

    /// Number of parallel trajectories to use, or `None` for the serial orbit.
    /// Clamped to `n / 10,000` so the 1,000-point burn-in of each stays cheap.
    fn orbit_split(&self) -> Option<usize> {
        let k = self.parallel_orbit?.min((self.n / MIN_TRAJECTORY).max(1));
        (k > 1).then_some(k)
    }

    /// Orbit, range and binning of the default pipeline.
    ///
    /// The first 1,000,000 points set the range (percentiles); the rest are
    /// generated in chunks and binned in parallel. The orbit itself is serial.
    fn default_counts(&self) -> Result<Counts, String> {
        let a = &self.a;
        let n = self.n;
        let img_size = self.img_size;
        let (x0, y0) = self.xy;

        let mut timings = Timings::default();
        let stage = Instant::now();
        // the points that set the range; fewer when the whole orbit is shorter
        let phase_one = PHASE_ONE.min(n.max(1));
        let mut xy: Vec<(f64, f64)> = Vec::with_capacity(phase_one);
        xy.push((x0, y0));

        let mut x1: f64 = x0;
        let mut y1: f64 = y0;

        // compute a burn-in of 1000 samples and discard
        let diverged = |timings: &mut Timings, n: usize| {
            timings.orbit = stage.elapsed();
            Counts {
                counts: Array2::zeros((0, 0)),
                n,
                finite: false,
                timings: *timings,
            }
        };
        let progress_bar = Progress::new((n / 100_000) as u64);
        let mut par = None;
        if let Some(k) = self.orbit_split() {
            let Some(mut gen) = ParallelOrbit::new(a, self.xy, k) else {
                return Ok(diverged(&mut timings, 0));
            };
            xy.resize(phase_one, (0.0, 0.0));
            if !gen.fill(&mut xy) {
                return Ok(diverged(&mut timings, 0));
            }
            par = Some(gen);
        } else {
            for i in 1..1000 {
                (x1, y1) = sprott_7e(a, &x1, &y1);
                if !x1.is_finite() || !y1.is_finite() {
                    return Ok(diverged(&mut timings, i));
                }
            }

            // compute the points that set the range
            for i in 1..phase_one {
                if i % 100_000 == 0 {
                    progress_bar.inc(1);
                }
                (x1, y1) = sprott_7e(a, &x1, &y1);
                if !x1.is_finite() || !y1.is_finite() {
                    return Ok(diverged(&mut timings, i));
                }
                xy.push((x1, y1));
            }
        }
        timings.orbit = stage.elapsed();

        // compute percentiles
        let stage = Instant::now();
        let (xrange, yrange) = compute_percentiles(&xy, &self.perc_threshold);
        check_not_collapsed((xrange[0], xrange[1]), (yrange[0], yrange[1]))?;
        let xr = (xrange[0], xrange[1]);
        let yr = (yrange[0], yrange[1]);
        timings.range = stage.elapsed();

        // discretize into image
        let stage = Instant::now();
        let counter = Counter::new(img_size.0, img_size.1);
        bin_par(&counter, xy.par_iter().copied(), |p| {
            cell_default(p, xr, yr, img_size)
        });
        timings.discretise = stage.elapsed();
        drop(xy);

        // the remaining points, a chunk at a time
        let mut buf: Vec<(f64, f64)> = Vec::with_capacity(CHUNK.min(n.saturating_sub(phase_one)));
        let mut done = phase_one;
        while done < n {
            let len = CHUNK.min(n - done);
            let stage = Instant::now();
            buf.clear();
            let mut finite = true;
            if let Some(gen) = par.as_mut() {
                buf.resize(len, (0.0, 0.0));
                finite = gen.fill(&mut buf);
            } else {
                for _ in 0..len {
                    (x1, y1) = sprott_7e(a, &x1, &y1);
                    buf.push((x1, y1));
                }
            }
            timings.orbit += stage.elapsed();
            if !finite {
                progress_bar.finish();
                return Ok(Counts {
                    counts: Array2::zeros((0, 0)),
                    n: done,
                    finite: false,
                    timings,
                });
            }

            let stage = Instant::now();
            bin_par(&counter, buf.par_iter().copied(), |p| {
                cell_default(p, xr, yr, img_size)
            });
            timings.discretise += stage.elapsed();
            progress_bar.inc((len / 100_000) as u64);
            done += len;
        }
        progress_bar.finish();

        Ok(Counts {
            counts: counter.into_array(img_size.0, img_size.1),
            n,
            finite: true,
            timings,
        })
    }
}

/// Hit counts from the pipeline, before colouring
pub struct Counts {
    /// hit counts, `width x height`; the count of pixel `(x, y)` is `counts[[x, y]]`
    pub counts: Array2<u32>,
    /// number of points computed (where it stopped, if it diverged)
    pub n: usize,
    /// `false` if the orbit diverged; `counts` is then empty
    pub finite: bool,
    /// time spent in each stage so far
    pub timings: Timings,
}

/// Progress reporting; does nothing unless the `progress` feature is on.
struct Progress(#[cfg(feature = "progress")] indicatif::ProgressBar);

impl Progress {
    #[allow(unused_variables)]
    fn new(len: u64) -> Self {
        Progress(#[cfg(feature = "progress")] indicatif::ProgressBar::new(len))
    }
    #[allow(unused_variables)]
    fn inc(&self, delta: u64) {
        #[cfg(feature = "progress")]
        self.0.inc(delta);
    }
    fn finish(&self) {
        #[cfg(feature = "progress")]
        self.0.finish();
    }
}

/// Points that set the range before binning starts
const PHASE_ONE: usize = 1_000_000;

/// Points generated per chunk once the range is known
const CHUNK: usize = 1 << 20;

/// Fewest points per trajectory when the orbit is split (see `orbit_split`)
const MIN_TRAJECTORY: usize = 10_000;

/// Independent trajectories that continue across chunks, filled in parallel.
///
/// Trajectory `i` of `k` starts at the start point scaled by `1 + f` in x and
/// `1 - f` in y, with `f = 0.1 i / k` (so within 10% of it, however large `k`
/// is, and trajectory 0 is unchanged), then discards 1,000 points.
struct ParallelOrbit<'a> {
    a: &'a Array1<f64>,
    states: Vec<(f64, f64)>,
}

impl<'a> ParallelOrbit<'a> {
    /// `None` if any trajectory diverges during burn-in
    fn new(a: &'a Array1<f64>, (x0, y0): (f64, f64), k: usize) -> Option<Self> {
        let mut states: Vec<(f64, f64)> = (0..k)
            .map(|i| {
                let f = 0.1 * i as f64 / k as f64;
                (x0 * (1.0 + f), y0 * (1.0 - f))
            })
            .collect();
        let ok = states
            .par_iter_mut()
            .map(|s| {
                for _ in 0..1000 {
                    *s = sprott_7e(a, &s.0, &s.1);
                }
                s.0.is_finite() && s.1.is_finite()
            })
            .all(|ok| ok);
        ok.then_some(ParallelOrbit { a, states })
    }

    /// Fill `out` with the next points, an equal share per trajectory. Returns
    /// `false` if any point is not finite.
    fn fill(&mut self, out: &mut [(f64, f64)]) -> bool {
        let share = out.len().div_ceil(self.states.len()).max(1);
        let a = self.a;
        self.states
            .par_iter_mut()
            .zip(out.par_chunks_mut(share))
            .map(|(s, o)| {
                let mut ok = true;
                for slot in o.iter_mut() {
                    *s = sprott_7e(a, &s.0, &s.1);
                    ok &= s.0.is_finite() && s.1.is_finite();
                    *slot = *s;
                }
                ok
            })
            .all(|ok| ok)
    }
}

/// Hit counts that parallel tasks can increment.
///
/// Integer addition commutes, so the result does not depend on thread count
/// or order. Atomics keep memory at one matrix, which matters at 16K.
struct Counter {
    cells: Vec<AtomicU32>,
    cols: usize,
}

impl Counter {
    fn new(rows: usize, cols: usize) -> Self {
        Counter {
            cells: (0..rows * cols).map(|_| AtomicU32::new(0)).collect(),
            cols,
        }
    }

    fn add(&self, row: usize, col: usize) {
        self.cells[row * self.cols + col].fetch_add(1, Ordering::Relaxed);
    }

    fn into_array(self, rows: usize, cols: usize) -> Array2<u32> {
        let cells = self.cells.into_iter().map(AtomicU32::into_inner).collect();
        Array2::from_shape_vec((rows, cols), cells).expect("rows * cols cells")
    }
}

/// Bin points in parallel. `cell` gives the matrix cell of a point, or `None`
/// to drop it. Returns the number of points binned.
fn bin_par<I, F>(counter: &Counter, points: I, cell: F) -> usize
where
    I: ParallelIterator<Item = (f64, f64)>,
    F: Fn((f64, f64)) -> Option<(usize, usize)> + Sync + Send,
{
    points
        .filter_map(cell)
        .map(|(r, c)| counter.add(r, c))
        .count()
}

/// Matrix cell of an xy element in the default pipeline, or `None` if it falls
/// outside the image. Truncates.
fn cell_default(
    (x, y): (f64, f64),
    xfrom: (f64, f64),
    yfrom: (f64, f64),
    (w, h): (usize, usize),
) -> Option<(usize, usize)> {
    let (xsize, ysize) = (w - 1, h - 1);
    let xx: isize = remap(&x, xfrom, (0.0, xsize as f64)) as isize;
    let yy: isize = remap(&y, yfrom, (0.0, ysize as f64)) as isize;
    (xx >= 0 && yy >= 0 && xx <= xsize as isize && yy <= ysize as isize)
        .then_some((xx as usize, yy as usize))
}

/// Convert array to RgbaImage
///
/// Pixel `(x, y)` is the count at `image[[x, y]]`.
pub fn convert_to_image(image: Array2<u32>, gradient: &Gradient, timings: &mut Timings) -> RgbaImage {
    /// rows of the output handled per task; the transposed read is tiled by this
    const BAND: usize = 64;

    let stage = Instant::now();
    let (w, h) = image.dim();
    let image = image.as_standard_layout();
    let counts = image.as_slice().expect("standard layout is contiguous");
    // log1p of the count, rescaled over the non-empty pixels so that the whole
    // gradient is used. ln_1p is increasing, so the
    // extremes come from the extreme counts.
    let (lo_c, hi_c) = nonzero_range(counts).unwrap_or((0, 0));
    let (lo, hi) = ((lo_c as f64).ln_1p(), (hi_c as f64).ln_1p());
    timings.rescale = stage.elapsed();

    let stage = Instant::now();
    // empty pixels are white for gradients that start light (BuPu, Oranges, ...)
    // and black otherwise, so low counts stay faint against the background
    let [r, g, b, _] = gradient.at(0.0).to_rgba8();
    let light_start = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64 > 128.0;
    let background = if light_start {
        [255, 255, 255, 255]
    } else {
        [0, 0, 0, 255]
    };

    let mut imgbuf: RgbaImage = ImageBuffer::new(w as u32, h as u32);
    if w > 0 && h > 0 {
        imgbuf
            .par_chunks_mut(w * 4 * BAND)
            .enumerate()
            .for_each(|(band, chunk)| {
                let y0 = band * BAND;
                for x in 0..w {
                    for (dy, row) in chunk.chunks_exact_mut(w * 4).enumerate() {
                        let c = counts[x * h + y0 + dy];
                        row[x * 4..x * 4 + 4].copy_from_slice(&if c == 0 {
                            background
                        } else {
                            gradient.at(scale_01((c as f64).ln_1p(), lo, hi)).to_rgba8()
                        });
                    }
                }
            });
    }
    timings.colour = stage.elapsed();
    imgbuf
}

/// Save RgbaImage to png file, returning the time taken
///
/// Fast compression with no row filter: on these noisy density images the
/// filters cost time and make the file bigger (see `docs/performance.md`).
pub fn save_attractor(imgbuf: RgbaImage, filename: &str) -> Result<Duration, String> {
    let stage = Instant::now();
    let file = std::io::BufWriter::new(
        std::fs::File::create(filename).map_err(|e| format!("cannot create {filename}: {e}"))?,
    );
    PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter)
        .write_image(
            imgbuf.as_raw(),
            imgbuf.width(),
            imgbuf.height(),
            ColorType::Rgba8,
        )
        .map_err(|e| format!("cannot write {filename}: {e}"))?;
    Ok(stage.elapsed())
}

/// Fail if the orbit has collapsed onto a fixed point or a tiny cycle, so that
/// the range of either axis is (numerically) zero and there is nothing to draw.
fn check_not_collapsed(xr: (f64, f64), yr: (f64, f64)) -> Result<(), String> {
    let collapsed = |(lo, hi): (f64, f64)| hi - lo <= 1e-9 * lo.abs().max(hi.abs()).max(1.0);
    if collapsed(xr) || collapsed(yr) {
        return Err(format!(
            "The orbit collapsed to a fixed point or a tiny cycle (x range {xr:?}, y range {yr:?}); these coefficients have no attractor to draw"
        ));
    }
    Ok(())
}

/// Given a vector and a threshold, compute percentile cutofss
fn compute_percentiles(xy: &[(f64, f64)], perc_threshold: &f64) -> (Vec<f64>, Vec<f64>) {
    let percs = [
        (1.0 - perc_threshold) / 2.0,
        perc_threshold + (1.0 - perc_threshold) / 2.0,
    ];
    let axis = |pick: fn(&(f64, f64)) -> f64| {
        let mut perc = Percentiles::new();
        for e in xy {
            perc.add(&pick(e));
        }
        perc.percentiles(percs).unwrap().unwrap()
    };
    rayon::join(|| axis(|e| e.0), || axis(|e| e.1))
}

/// Position of a value in `[lo, hi]` as a fraction
fn scale_01(x: f64, lo: f64, hi: f64) -> f64 {
    if lo == hi {
        return 0.5;
    }
    (x - lo) / (hi - lo)
}

/// Smallest nonzero and largest value of a slice of counts, computed in parallel
fn nonzero_range<T: Copy + Ord + Send + Sync + Default>(counts: &[T]) -> Option<(T, T)> {
    let zero = T::default();
    counts
        .par_iter()
        .copied()
        .filter(|&c| c != zero)
        .fold(
            || None,
            |acc: Option<(T, T)>, c| match acc {
                None => Some((c, c)),
                Some((lo, hi)) => Some((lo.min(c), hi.max(c))),
            },
        )
        .reduce(
            || None,
            |a, b| match (a, b) {
                (None, x) | (x, None) => x,
                (Some((l1, h1)), Some((l2, h2))) => Some((l1.min(l2), h1.max(h2))),
            },
        )
}

/// ColorBrewer palettes available by name
#[rustfmt::skip]
const BREWER: [(&str, &[&str]); 8] = [
    ("BuPu", &["#f7fcfd", "#e0ecf4", "#bfd3e6", "#9ebcda", "#8c96c6", "#8c6bb1", "#88419d", "#810f7c", "#4d004b"]),
    ("Oranges", &["#fff5eb", "#fee6ce", "#fdd0a2", "#fdae6b", "#fd8d3c", "#f16913", "#d94801", "#a63603", "#7f2704"]),
    ("Reds", &["#fff5f0", "#fee0d2", "#fcbba1", "#fc9272", "#fb6a4a", "#ef3b2c", "#cb181d", "#a50f15", "#67000d"]),
    ("Greens", &["#f7fcf5", "#e5f5e0", "#c7e9c0", "#a1d99b", "#74c476", "#41ab5d", "#238b45", "#006d2c", "#00441b"]),
    ("Blues", &["#f7fbff", "#deebf7", "#c6dbef", "#9ecae1", "#6baed6", "#4292c6", "#2171b5", "#08519c", "#08306b"]),
    ("BuGn", &["#f7fcfd", "#e5f5f9", "#ccece6", "#99d8c9", "#66c2a4", "#41ae76", "#238b45", "#006d2c", "#00441b"]),
    ("Spectral", &["#9e0142", "#d53e4f", "#f46d43", "#fdae61", "#fee08b", "#ffffbf", "#e6f598", "#abdda4", "#66c2a5", "#3288bd", "#5e4fa2"]),
    ("PuOr", &["#7f3b08", "#b35806", "#e08214", "#fdb863", "#fee0b6", "#f7f7f7", "#d8daeb", "#b2abd2", "#8073ac", "#542788", "#2d004b"]),
];

/// Look up a gradient by name.
///
/// ColorBrewer names (`BuPu`, `Oranges`, ...) are interpolated linearly in Lab
/// space. Other names are `colorgrad` presets
/// such as `turbo` or `viridis`. Names are case-insensitive.
pub fn gradient_by_name(name: &str) -> Result<Gradient, String> {
    let lower = name.to_lowercase();
    if let Some((_, colours)) = BREWER.iter().find(|(n, _)| n.to_lowercase() == lower) {
        return lab_gradient(colours);
    }
    match lower.as_str() {
        "turbo" => Ok(colorgrad::turbo()),
        "viridis" => Ok(colorgrad::viridis()),
        "magma" => Ok(colorgrad::magma()),
        "inferno" => Ok(colorgrad::inferno()),
        "plasma" => Ok(colorgrad::plasma()),
        "cividis" => Ok(colorgrad::cividis()),
        _ => {
            let brewer: Vec<&str> = BREWER.iter().map(|(n, _)| *n).collect();
            Err(format!(
                "unknown gradient '{name}': use turbo, viridis, magma, inferno, plasma, cividis, or one of {}",
                brewer.join(", ")
            ))
        }
    }
}

/// sRGB (0..1) to CIELAB, D65 white point
fn srgb_to_lab(rgb: [f64; 3]) -> [f64; 3] {
    let lin = |c: f64| {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let (r, g, b) = (lin(rgb[0]), lin(rgb[1]), lin(rgb[2]));
    let x = (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / 0.95047;
    let y = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
    let z = (0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / 1.08883;
    let f = |t: f64| {
        if t > 216.0 / 24389.0 {
            t.cbrt()
        } else {
            (24389.0 / 27.0 * t + 16.0) / 116.0
        }
    };
    let (fx, fy, fz) = (f(x), f(y), f(z));
    [116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz)]
}

/// CIELAB (D65) to sRGB (0..1), clamped to the gamut
fn lab_to_srgb(lab: [f64; 3]) -> [f64; 3] {
    let fy = (lab[0] + 16.0) / 116.0;
    let fx = fy + lab[1] / 500.0;
    let fz = fy - lab[2] / 200.0;
    let finv = |t: f64| {
        if t.powi(3) > 216.0 / 24389.0 {
            t.powi(3)
        } else {
            (116.0 * t - 16.0) * 27.0 / 24389.0
        }
    };
    let (x, y, z) = (finv(fx) * 0.95047, finv(fy), finv(fz) * 1.08883);
    let r = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let g = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let b = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;
    let gamma = |c: f64| {
        let c = c.clamp(0.0, 1.0);
        if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    };
    [gamma(r), gamma(g), gamma(b)]
}

/// Gradient through the given colours, interpolated linearly in Lab space. `colorgrad` 0.6 has no Lab blend mode, so Lab
/// is sampled at 256 stops, which is within a colour level of exact.
fn lab_gradient(colours: &[&str]) -> Result<Gradient, String> {
    let labs: Vec<[f64; 3]> = colours
        .iter()
        .map(|c| {
            let c = Color::from_html(c).map_err(|e| e.to_string())?;
            Ok(srgb_to_lab([c.r, c.g, c.b]))
        })
        .collect::<Result<_, String>>()?;
    let segments = (labs.len() - 1) as f64;
    let stops: Vec<Color> = (0..256)
        .map(|i| {
            let t = i as f64 / 255.0 * segments;
            let k = (t.floor() as usize).min(labs.len() - 2);
            let f = t - k as f64;
            let lab = std::array::from_fn(|d| labs[k][d] + (labs[k + 1][d] - labs[k][d]) * f);
            let [r, g, b] = lab_to_srgb(lab);
            Color::new(r, g, b, 1.0)
        })
        .collect();
    colorgrad::CustomGradient::new()
        .colors(&stops)
        .build()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// deterministic pseudo-random counts, about a third of them zero
    fn counts(rows: usize, cols: usize) -> Array2<u32> {
        let mut state = 12345u64;
        Array2::from_shape_fn((rows, cols), |_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let v = (state >> 33) as u32;
            if v.is_multiple_of(3) {
                0
            } else {
                v % 5000
            }
        })
    }

    fn gradients() -> Vec<Gradient> {
        vec![
            colorgrad::viridis(),
            gradient_by_name("BuPu").unwrap(),
            gradient_by_name("turbo").unwrap(),
        ]
    }

    /// serial per-pixel reference for `convert_to_image` (pixel (x, y) is `c[[x, y]]`)
    fn convert_reference(c: &Array2<u32>, g: &Gradient) -> RgbaImage {
        let (w, h) = c.dim();
        let logged = c.mapv(|v| (v as f64).ln_1p());
        let nz: Vec<f64> = logged.iter().cloned().filter(|&v| v != 0.0).collect();
        let lo = nz.iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = nz.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let [r, gg, b, _] = g.at(0.0).to_rgba8();
        let light = 0.299 * r as f64 + 0.587 * gg as f64 + 0.114 * b as f64 > 128.0;
        let bg = if light {
            [255, 255, 255, 255]
        } else {
            [0, 0, 0, 255]
        };
        let mut img = RgbaImage::new(w as u32, h as u32);
        for x in 0..w {
            for y in 0..h {
                let v = logged[[x, y]];
                let px = if v == 0.0 {
                    bg
                } else {
                    g.at(scale_01(v, lo, hi)).to_rgba8()
                };
                img.put_pixel(x as u32, y as u32, image::Rgba(px));
            }
        }
        img
    }

    #[test]
    fn convert_to_image_matches_serial_reference() {
        let c = counts(131, 257);
        for g in gradients() {
            let got = convert_to_image(c.clone(), &g, &mut Timings::default());
            assert_eq!(got.as_raw(), convert_reference(&c, &g).as_raw());
        }
    }

    #[test]
    fn tiny_images_do_not_panic() {
        for (r, c) in [(1, 1), (1, 9), (9, 1)] {
            let m = counts(r, c).mapv(|v| v + 1);
            let g = colorgrad::viridis();
            let img = convert_to_image(m.clone(), &g, &mut Timings::default());
            assert_eq!(img.dimensions(), (r as u32, c as u32));
            assert_eq!(img.as_raw(), convert_reference(&m, &g).as_raw());
        }
    }

    #[test]
    fn saved_png_roundtrips() {
        let c = counts(48, 64).mapv(|v| v + 1);
        let img = convert_to_image(c, &colorgrad::viridis(), &mut Timings::default());
        let path = std::env::temp_dir().join("roundtrip.png");
        let path = path.to_str().unwrap();
        save_attractor(img.clone(), path).unwrap();
        assert_eq!(image::open(path).unwrap().to_rgba8().as_raw(), img.as_raw());
    }

    /// serial per-point reference for the binners
    fn bin_reference(
        pts: &[(f64, f64)],
        dim: (usize, usize),
        cell: impl Fn((f64, f64)) -> Option<(usize, usize)>,
    ) -> Array2<u32> {
        let mut m = Array2::zeros(dim);
        for &p in pts {
            if let Some(c) = cell(p) {
                m[[c.0, c.1]] += 1;
            }
        }
        m
    }

    fn orbit(n: usize) -> Vec<(f64, f64)> {
        let a = Attractor::default().a;
        let (mut x, mut y) = (1.0, 1.0);
        (0..n)
            .map(|_| {
                (x, y) = sprott_7e(&a, &x, &y);
                (x, y)
            })
            .collect()
    }

    #[test]
    fn parallel_binning_equals_serial() {
        let pts = orbit(2_000_000);
        let (xr, yr) = compute_percentiles(&pts, &0.9);
        let (xr, yr) = ((xr[0], xr[1]), (yr[0], yr[1]));
        for dim in [(257, 131), (1, 1), (1, 7), (7, 1)] {
            let want = bin_reference(&pts, dim, |p| cell_default(p, xr, yr, dim));
            let counter = Counter::new(dim.0, dim.1);
            bin_par(&counter, pts.par_iter().copied(), |p| {
                cell_default(p, xr, yr, dim)
            });
            assert_eq!(counter.into_array(dim.0, dim.1), want, "{dim:?}");
        }
    }

    /// the default pipeline's counts as the original serial code computed them
    fn default_counts_reference(att: &Attractor) -> Array2<u32> {
        let (w, h) = att.img_size;
        let (mut x, mut y) = att.xy;
        let mut xy = vec![(x, y)];
        for _ in 1..1000 {
            (x, y) = sprott_7e(&att.a, &x, &y);
        }
        let phase_one = 1_000_000.min(att.n.max(1));
        for _ in 1..phase_one {
            (x, y) = sprott_7e(&att.a, &x, &y);
            xy.push((x, y));
        }
        let (xr, yr) = compute_percentiles(&xy, &att.perc_threshold);
        let (xr, yr) = ((xr[0], xr[1]), (yr[0], yr[1]));
        let mut m = bin_reference(&xy, (w, h), |p| cell_default(p, xr, yr, (w, h)));
        for _ in phase_one..att.n {
            (x, y) = sprott_7e(&att.a, &x, &y);
            if let Some(c) = cell_default((x, y), xr, yr, (w, h)) {
                m[[c.0, c.1]] += 1;
            }
        }
        m
    }

    #[test]
    fn default_counts_match_serial_reference() {
        for n in [500_000, 2_500_000] {
            let att = Attractor {
                n,
                img_size: (97, 53),
                ..Default::default()
            };
            let got = att.default_counts().unwrap();
            assert!(got.finite);
            assert_eq!(got.counts, default_counts_reference(&att), "n = {n}");
            assert!(got.timings.discretise > Duration::ZERO);
        }
    }

    #[test]
    fn short_orbits_give_exactly_n_points() {
        // n below the 1,000,000 range-setting phase used to be padded up to it
        for k in [None, Some(4)] {
            let att = Attractor {
                n: 50_000,
                img_size: (40, 30),
                parallel_orbit: k,
                ..Default::default()
            };
            let got = att.counts().unwrap();
            assert!(got.finite);
            let total: u64 = got.counts.iter().map(|&c| u64::from(c)).sum();
            // the percentile range drops some points, never adds any
            assert!(total > 0 && total <= 50_000, "k {k:?}: total {total}");
        }
    }

    #[test]
    fn many_parallel_trajectories_all_start_sanely() {
        // trajectory 1000 used to start at y = 0 and later ones with y flipped
        let a = Attractor::default().a;
        for k in [1000, 1001, 2000, 5000] {
            assert!(ParallelOrbit::new(&a, (1.0, 1.0), k).is_some(), "k = {k}");
        }
    }
}
