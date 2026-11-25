# Use palette from ColorBrewer to recolour the image.

You can use any of the colours defined by the
[`scales::col_numeric()`](https://scales.r-lib.org/reference/col_numeric.html)
function.

## Usage

``` r
recolour(
  x,
  palette = "Blues",
  zero_colour = NA,
  invert = FALSE,
  trans = "log1p"
)

recolor(
  x,
  palette = "Blues",
  zero_colour = NA,
  invert = FALSE,
  trans = "log1p"
)
```

## Arguments

- x:

  Matrix

- palette:

  ColorBrewer palette. This value is passed to
  [`scales::col_numeric()`](https://scales.r-lib.org/reference/col_numeric.html)

- zero_colour:

  Colour to use for zero and NA values

- invert:

  If `TRUE`, inverts the colour scale

- trans:

  Transformation function, defaulting to
  [`log1p()`](https://rdrr.io/r/base/Log.html)

## Value

Matrix

## Details

The palette argument can be any of the following:

1.  A character vector of RGB or named colours. Examples:
    [`palette()`](https://rdrr.io/r/grDevices/palette.html),
    `c("#000000", "#0000FF", "#FFFFFF")`, `topo.colors(10)` 2. The name
    of an `RColorBrewer` palette, e.g. `"BuPu"` or `"Greens"`. 3. A
    function that receives a single value between 0 and 1 and returns a
    colour. Examples:
    `colorRamp(c("#000000", "#FFFFFF"), interpolate="spline")`.

## Examples

``` r
# Generate the seed vector (14 random values)
a <- seed_sprott_7e(fast_forward = 429)

# Generate points, trim at the 0.03 quantile, and discretize to a 600x900 matrix
dat <- attractor_sprott_7e(a, n = 1e6, qs = 0.03, dims = c(600, 900))

# Set empty plot margins
par(mar = rep(0, 4), mai = rep(0, 4))

# Plot using various palettes from `RColorBrewer`
plot(recolour(dat, palette = "Oranges"))

plot(recolour(dat, palette = "Greens"))


plot(recolour(dat, palette = "Oranges", zero_colour = "#FFFAF6"))

plot(recolour(dat, palette = "Oranges", zero_colour = "grey20", invert = TRUE))
```
