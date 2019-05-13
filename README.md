
<!-- README.md is generated from README.Rmd. Please edit that file -->

# attractor <img src="man/figures/attractor-logo.png" align="right" width="120" />

<!-- badges: start -->

<!-- badges: end -->

The goal of `attractor` is to generate and plot strange attractors.

Using this package is fast, because:

1.  The attractor algorithm uses `Rcpp` for speed.
2.  The resulting attractor is discretized (binned) into a much smaller
    matrix, making plotting very fast.

## Installation

You can install the development version from
[GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("andrie/attractor")
```

## Example

Create a strange attractor with 10 million points, then discretize to a
matrix with dimensions `400 x 400`:

``` r
library(magrittr)
library(attractor)

a <- seed_sprott_7e(21031)
#> setting seed

dat <- 
  a %>% 
  attractor_sprott_7e(1e6, dims = c(400, 400))
str(dat)
#>  num [1:400, 1:400] 0 0 0 0 0 0 0 0 0 0 ...
```

## Plotting a strange attractor

``` r
par(mar = rep(0, 4), mai = rep(0, 4))
dat %>% 
  recolour() %>% 
  plot()
```

<img src="man/figures/README-unnamed-chunk-3-1.png" height="400px" />

## Recolour the plot

Use `recolour()` to change the colours. You can use any of the colours
defined by the `scales::col_numeric()` function. From the help of
`?scales::col_numeric()`:

The palette argument can be any of the following:

1.  A character vector of RGB or named colours. Examples: `palette()`,
    `c("#000000", "#0000FF", "#FFFFFF")`, `topo.colors(10)`

2.  The name of an `RColorBrewer` palette, e.g. `"BuPu"` or `"Greens"`.

3.  A function that receives a single value between 0 and 1 and returns
    a colour. Examples: `colorRamp(c("#000000", "#FFFFFF"),
    interpolate="spline")`.

Use the `Oranges` palette:

``` r
dat %>% 
  recolour("Oranges") %>%
  plot()
```

<img src="man/figures/README-unnamed-chunk-4-1.png" height="400px" />

To change the background colour, specify a `zero_colour`:

``` r
dat %>% 
  recolour("Oranges", zero_colour = "#FFFAF6") %>%
  plot()
```

<img src="man/figures/README-unnamed-chunk-5-1.png" height="400px" />

You can also invert the palette

``` r
dat %>% 
  recolour("Oranges", zero_colour = "grey20", invert = TRUE) %>%
  plot()
```

<img src="man/figures/README-unnamed-chunk-6-1.png" height="400px" />

Try the `Spectral` palette:

``` r
dat %>% 
  recolour("Spectral", zero_colour = "#9E0142") %>%
  plot()
```

<img src="man/figures/README-unnamed-chunk-7-1.png" height="400px" />

With inversion:

``` r
dat %>% 
  recolour("Spectral", invert = TRUE, zero_colour = "#5E4FA2") %>%
  plot()
```

<img src="man/figures/README-unnamed-chunk-8-1.png" height="400px" />
