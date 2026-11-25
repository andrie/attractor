#' Generate strange attractor.
#'
#' @param a Numeric vector that contains the coefficients for computing the
#'   attractor
#'
#' @param n Number of points to generate
#'
#' @param x0 Initial value for x
#' @param y0 Initial value for y
#'
#' @param dims Dimensions of resulting matrix, only used if `discretize == TRUE`
#'
#' @param n_discretize Ignored; the range is set from the first 1e6 points.
#'   Retained for backward compatibility.
#'
#' @param qs Quantile cutoff: a single number in `[0, 0.5)`. The range of each
#'   axis is set at the `qs` and `1 - qs` percentiles.
#' @param progress Ignored; retained for backward compatibility
#'
#' @param parallel_orbit Number of independent orbit trajectories that run in
#'   parallel (one per thread by default). Use `1` for the serial orbit. The
#'   attractor looks the same either way, but the counts differ in detail, and
#'   for a given number of trajectories the result is deterministic.
#'
#' @param discretize If `TRUE` discretizes to a matrix with dimensions `dims`,
#'   else returns a data frame with one row for every data point
#'
#' @return if `discretize == TRUE` a matrix with dimensions `dims`, else a data
#'   frame with columns `x` and `y`, and `n` rows.
#'
#' @importFrom assertthat assert_that
#'
#' @keywords Internal
#'
attractor_generic <- function(
  a,
  n,
  x0 = 1,
  y0 = 1,
  dims = c(600, 600),
  progress = (n >= 1e5),
  n_discretize = 1e6,
  qs = 0.05,
  parallel_orbit = default_parallel_orbit(),
  discretize = TRUE
) {
  assert_that(is.vector(a))
  assert_that(is.numeric(a))
  assert_that(length(a) == 14)
  assert_that(is.numeric(n))
  assert_that(length(n) == 1)
  assert_that(n > 0)
  assert_that(is.numeric(x0))
  assert_that(length(x0) == 1)
  assert_that(is.numeric(y0))
  assert_that(length(y0) == 1)
  assert_that(is.numeric(qs))
  assert_that(length(qs) == 1)
  assert_that(qs >= 0 && qs < 0.5)
  assert_that(is.numeric(dims))
  assert_that(length(dims) == 2)
  assert_that(is.numeric(parallel_orbit))
  assert_that(length(parallel_orbit) == 1)
  assert_that(parallel_orbit >= 1)

  if (discretize) {
    z <- attractor_matrix(
      a,
      n = n,
      x0 = x0,
      y0 = y0,
      dims = dims,
      perc_threshold = 1 - 2 * qs,
      parallel_orbit = parallel_orbit
    )
    class(z) <- c("attractor", "matrix")
  } else {
    z <- attractor_orbit(a, n = n, x0 = x0, y0 = y0)
    class(z) <- c("attractor", "data.frame")
  }
  z
}

#' Default number of parallel orbit trajectories: one per thread.
#' @noRd
default_parallel_orbit <- function() {
  max(1, parallel::detectCores(logical = TRUE), na.rm = TRUE)
}
