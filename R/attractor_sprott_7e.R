#' Generate strange attractor using equation 7E of Sprott.
#'
#' This attractor is based on Equation 7E of Sprott (see references).
#'
#' \deqn{x_{i+1} = a_{1} + a_{2} * x_{i} +  a_{3} * y_{i} +  a_{4} * |x_{i}|^{a5}  +  a6 * |y_{i}|^{a7}}
#'
#' \deqn{y_{i+1} = a_{8} + a_{9} * x_{i} +  a_{10} * y_{i} +  a_{11} * |x_{i}|^{a12}  +  a13 * |y_{i}|^{a14}}
#'
#' @references Julien C. Sprott, "Strange Attractors: Creating Patterns in Chaos", page 418, Equation 7e, http://sprott.physics.wisc.edu/fractals/booktext/sabook.pdf
#'
#' @param a Numeric vector of length 14
#' @param n Number of points to generate
#' @param x0 Initial value for x
#' @param y0 Initial value for y
#' @param dims Dimensions of resulting matrix
#' @param n_discretize Ignored; the range is set from the first 1e6 points
#' @param qs quantile cutoff, a single number in `[0, 0.5)`
#' @param progress Ignored; retained for backward compatibility
#' @param parallel_orbit Number of parallel orbit trajectories (one per thread by default); `1` for the serial orbit
#' @param discretize If `TRUE` discretizes to a matrix with dimensions `dims` else a data frame
#'
#' @return if `discretize == TRUE` a matrix with dimensions `dims`, else a data frame with columns `x` and `y`, and `n` rows.
#' @importFrom assertthat assert_that
#' @export
#' @family sprott functions
#'
#' @example inst/examples/example_sprott_7e.R
#'
attractor_sprott_7e <- function(
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
  assert_that(length(a) == 14)
  attractor_generic(
    a = a,
    n = n,
    x0 = x0,
    y0 = y0,
    dims = dims,
    progress = progress,
    n_discretize = n_discretize,
    qs = qs,
    parallel_orbit = parallel_orbit,
    discretize = discretize
  )
}


#' Generate attractor for the Sprott 7e attractor.
#'
#' @param fast_forward "Index value" to retrieve
#' @param seed Original seed value, passed to [set.seed()]
#'
#' @importFrom stats runif rnorm
#' @export
#' @family sprott functions
seed_sprott_7e <- function(
  fast_forward,
  seed = if (!missing(fast_forward)) 1 else NA
) {
  if (!is.na(seed)) {
    # Seed locally: leave the caller's RNG state as we found it
    old <- get0(".Random.seed", envir = globalenv(), inherits = FALSE)
    on.exit(
      if (is.null(old)) {
        rm(".Random.seed", envir = globalenv())
      } else {
        assign(".Random.seed", old, envir = globalenv())
      },
      add = TRUE
    )
    set.seed(seed)
  }
  if (!missing(fast_forward)) {
    runif(2 * 14 * (fast_forward - 1))
  }
  rnorm(14, sd = 1)
}
