# Draw a Sprott 7e attractor straight to a PNG file.

Orbit, binning, colouring and PNG encoding all run in Rust, which makes
this the fast route to large images, for example for print. The PNG uses
fast compression with no row filter, which trades a larger file for
speed.

## Usage

``` r
png_sprott_7e(
  a,
  filename = "attractor.png",
  n = 1e+07,
  x0 = 1,
  y0 = 1,
  size = c(3840, 2160),
  gradient = "turbo",
  qs = 0.075,
  parallel_orbit = default_parallel_orbit()
)
```

## Arguments

- a:

  Numeric vector of length 14

- filename:

  Output file name.

- n:

  Number of points to generate

- x0:

  Initial value for x

- y0:

  Initial value for y

- size:

  Image size in pixels, `c(width, height)`.

- gradient:

  Gradient name: one of `"turbo"`, `"viridis"`, `"magma"`, `"inferno"`,
  `"plasma"`, `"cividis"`, or a ColorBrewer name such as `"BuPu"`,
  `"Oranges"`, `"Reds"`, `"Greens"`, `"Blues"`, `"BuGn"`, `"Spectral"`
  or `"PuOr"`.

- qs:

  quantile cutoff, a single number in `[0, 0.5)`

- parallel_orbit:

  Number of parallel orbit trajectories (one per thread by default); `1`
  for the serial orbit

## Value

The file name, invisibly.

## See also

Other sprott functions:
[`attractor_sprott_7e()`](https://andrie.github.io/attractor/reference/attractor_sprott_7e.md),
[`seed_sprott_7e()`](https://andrie.github.io/attractor/reference/seed_sprott_7e.md)

## Examples

``` r
a <- seed_sprott_7e(fast_forward = 429)
f <- png_sprott_7e(a, tempfile(fileext = ".png"), n = 1e6, size = c(400, 300))
```
