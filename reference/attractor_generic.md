# Generate strange attractor.

Generate strange attractor.

## Usage

``` r
attractor_generic(
  a,
  n,
  x0 = 1,
  y0 = 1,
  dims = c(600, 600),
  progress = (n >= 1e+05),
  n_discretize = 1e+06,
  qs = 0.05,
  parallel_orbit = default_parallel_orbit(),
  discretize = TRUE
)
```

## Arguments

- a:

  Numeric vector that contains the coefficients for computing the
  attractor

- n:

  Number of points to generate

- x0:

  Initial value for x

- y0:

  Initial value for y

- dims:

  Dimensions of resulting matrix, only used if `discretize == TRUE`

- progress:

  Ignored; retained for backward compatibility

- n_discretize:

  Ignored; the range is set from the first 1e6 points. Retained for
  backward compatibility.

- qs:

  Quantile cutoff: a single number in `[0, 0.5)`. The range of each axis
  is set at the `qs` and `1 - qs` percentiles.

- parallel_orbit:

  Number of independent orbit trajectories that run in parallel (one per
  thread by default). Use `1` for the serial orbit. The attractor looks
  the same either way, but the counts differ in detail, and for a given
  number of trajectories the result is deterministic.

- discretize:

  If `TRUE` discretizes to a matrix with dimensions `dims`, else returns
  a data frame with one row for every data point

## Value

if `discretize == TRUE` a matrix with dimensions `dims`, else a data
frame with columns `x` and `y`, and `n` rows.
