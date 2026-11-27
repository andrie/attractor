use attractor_core::{convert_to_image, gradient_by_name, save_attractor, Attractor};
use extendr_api::prelude::*;
use ndarray::Array1;

/// Build the engine configuration shared by the entry points below.
fn configure(
    a: &[f64],
    n: f64,
    x0: f64,
    y0: f64,
    size: &[f64],
    perc_threshold: f64,
    parallel_orbit: Option<f64>,
) -> std::result::Result<Attractor, Error> {
    if a.len() != 14 {
        return Err(Error::Other("`a` must have 14 coefficients".into()));
    }
    if size.len() != 2 || size.iter().any(|&d| !(d >= 1.0)) {
        return Err(Error::Other("`dims` must be two positive numbers".into()));
    }
    if !(n >= 1.0) {
        return Err(Error::Other("`n` must be at least 1".into()));
    }
    Ok(Attractor {
        a: Array1::from(a.to_vec()),
        n: n as usize,
        xy: (x0, y0),
        img_size: (size[0] as usize, size[1] as usize),
        perc_threshold,
        parallel_orbit: parallel_orbit.map(|k| k as usize),
        ..Default::default()
    })
}

/// Count matrix of a Sprott 7e attractor.
///
/// Orbit, percentile range and binning run in Rust. Returns a `dims[1] x dims[2]`
/// numeric matrix of hit counts, with attributes `xrange` and `yrange` holding
/// the frame (low and high percentile on each axis). Rows run along x, columns
/// along y. Errors if the orbit diverges or collapses.
///
/// @param a Numeric vector of 14 coefficients.
/// @param n Number of points to generate.
/// @param x0,y0 Initial values.
/// @param dims Dimensions of the matrix, `c(rows, cols)`.
/// @param perc_threshold Fraction of points kept around the median on each axis.
/// @param parallel_orbit Number of parallel trajectories, or `NULL` for serial.
/// @noRd
#[extendr]
fn attractor_matrix(
    a: &[f64],
    n: f64,
    x0: f64,
    y0: f64,
    dims: &[f64],
    perc_threshold: f64,
    parallel_orbit: Option<f64>,
) -> std::result::Result<Robj, Error> {
    let att = configure(a, n, x0, y0, dims, perc_threshold, parallel_orbit)?;
    let res = att.counts().map_err(Error::Other)?;
    if !res.finite {
        return Err(Error::Other(format!(
            "The orbit diverged to infinity after {} points; these coefficients have no attractor to draw",
            res.n
        )));
    }
    let (rows, cols) = att.img_size;
    // R matrices are column-major; row 0 is the largest x, as in earlier versions
    let mut out = vec![0.0; rows * cols];
    for ((x, y), &count) in res.counts.indexed_iter() {
        out[(rows - 1 - x) + y * rows] = f64::from(count);
    }
    let mut robj = Robj::from(out);
    let _ = robj.set_attrib(sym!(dim), Robj::from([rows as i32, cols as i32]));
    // the frame the points were binned into, so callers can size or crop the
    // image to the attractor's true proportions
    let _ = robj.set_attrib(sym!(xrange), Robj::from([res.xrange.0, res.xrange.1]));
    let _ = robj.set_attrib(sym!(yrange), Robj::from([res.yrange.0, res.yrange.1]));
    Ok(robj)
}

/// Raw orbit of a Sprott 7e attractor as a data frame with columns `x` and `y`.
///
/// @inheritParams attractor_matrix
/// @noRd
#[extendr]
fn attractor_orbit(a: &[f64], n: f64, x0: f64, y0: f64) -> std::result::Result<Robj, Error> {
    let att = configure(a, n, x0, y0, &[1.0, 1.0], 0.85, None)?;
    let pts = att.orbit().ok_or_else(|| {
        Error::Other("The orbit diverged; these coefficients have no attractor to draw".into())
    })?;
    let x: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let y: Vec<f64> = pts.iter().map(|p| p.1).collect();
    Ok(data_frame!(x = x, y = y))
}

/// Draw a Sprott 7e attractor straight to a PNG file.
///
/// Orbit, binning, colouring and PNG encoding all run in Rust (fast compression,
/// no row filter). Returns the file name.
///
/// @inheritParams attractor_matrix
/// @param size Image size in pixels, `c(width, height)`.
/// @param gradient Gradient name, e.g. `"turbo"` or a ColorBrewer name like `"BuPu"`.
/// @param filename Output file.
/// @noRd
#[extendr]
fn attractor_png(
    a: &[f64],
    n: f64,
    x0: f64,
    y0: f64,
    size: &[f64],
    perc_threshold: f64,
    parallel_orbit: Option<f64>,
    gradient: &str,
    filename: &str,
) -> std::result::Result<String, Error> {
    let mut att = configure(a, n, x0, y0, size, perc_threshold, parallel_orbit)?;
    att.gradient = gradient_by_name(gradient).map_err(Error::Other)?;
    let res = att.counts().map_err(Error::Other)?;
    if !res.finite {
        return Err(Error::Other(format!(
            "The orbit diverged to infinity after {} points; these coefficients have no attractor to draw",
            res.n
        )));
    }
    let mut timings = res.timings;
    let img = convert_to_image(res.counts, &att.gradient, &mut timings);
    save_attractor(img, filename).map_err(Error::Other)?;
    Ok(filename.to_string())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod attractor;
    fn attractor_matrix;
    fn attractor_orbit;
    fn attractor_png;
}
