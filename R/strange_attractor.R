#' Generate strange attractor.
#'
#' @param a Numeric vector of length 14
#' @param n Number of points to generate
#' @param x0 Initial value for x
#' @param y0 Initial value for y
#' @param dims Dimensions of resulting matrix
#' @param n_discretize Number of values to generate before computing the range
#' @param qs quantile cutoff
#' @param progress If TRUE, displays a progress bar
#' @param discretize If `TRUE` discretizes to a matrix with dimensions `dims` else a data frame
#'
#' @return if `discretize == TRUE` a matrix with dimensions `dims`, else a data frame with columns `x` and `y`, and `n` rows.
#' @importFrom assertthat assert_that
#' @keywords Internal
#'
attractor_generic <- function(
  a, n, x0 = 1, y0 = 1,
  dims = c(600, 600),
  progress = (n >= 1e5),
  n_discretize = 1e6,
  qs = qs,
  discretize = TRUE
){
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
  assert_that(is.numeric(n_discretize))
  assert_that(length(n_discretize) == 1)
  assert_that(is.numeric(qs))
  assert_that(length(qs) %in% c(1, 2, 4))
  assert_that(all(qs >= 0))
  assert_that(all(qs <= 1))


  assert_that(is.numeric(dims))
  assert_that(length(dims) == 2)

  if (length(qs) == 1) qs <- c(qs, 1 - qs, qs, 1 - qs)
  if (length(qs) == 2) qs <- c(qs[1], 1 - qs[1], 1 - qs[2], qs[2])
  # browser()

  if (discretize) {
    z <- strange_attractor_discretized_cpp(
      a, n = n,
      x0, y0,
      dims = dims,
      qs = qs,
      display_progress = progress,
      n_discretize = n_discretize
    )
  } else {
    z <- strange_attractor_cpp(
      a, n, x0, y0, display_progress = progress
    )
    class(z) <- c("attractor", "data.frame")
  }
  z

}

#' Generate strange attractor.
#'
#' Based on Equation 7E of Sprott.
#'
#' \deqn{x_{i+1} = a_{1} + a_{2} * x_{i} +  a_{3} * y_{i} +  a_{4} * |x_{i}|^{a5}  +  a6 * |y_{i}|^{a7}}
#'
#' \deqn{y_{i+1} = a_{8} + a_{9} * x_{i} +  a_{10} * y_{i} +  a_{11} * |x_{i}|^{a12}  +  a13 * |y_{i}|^{a14}}
#'
#' @param a Numeric vector of length 14
#' @param n Number of points to generate
#' @param x0 Initial value for x
#' @param y0 Initial value for y
#' @param dims Dimensions of resulting matrix
#' @param n_discretize Number of values to generate before computing the range
#' @param qs quantile cutoff
#' @param progress If TRUE, displays a progress bar
#' @param discretize If `TRUE` discretizes to a matrix with dimensions `dims` else a data frame
#'
#' @return if `discretize == TRUE` a matrix with dimensions `dims`, else a data frame with columns `x` and `y`, and `n` rows.
#' @importFrom assertthat assert_that
#' @references Julien C. Sprott, "Strange Attractors: Creating Patterns in Chaos", page 418, Equation 7e, http://sprott.physics.wisc.edu/fractals/booktext/sabook.pdf
#' @export
#'
attractor_sprott_7e <- function(
  a, n, x0 = 1, y0 = 1,
  dims = c(600, 600),
  progress = (n >= 1e5),
  n_discretize = 1e6,
  qs = 0.05,
  discretize = TRUE)
{
  assert_that(length(a) == 14)
  attractor_generic(
    a = a, n = n,
    x0 = x0, y0 = y0,
    dims = dims,
    progress = progress,
    n_discretize = n_discretize,
    qs = qs,
    discretize = discretize)
}




#' Generate attractor for the Sprott 7e attractor.
#'
#' @param fast_forward "Index value" to retrieve
#' @param seed Original seed value, passed to [set.seed()]
#'
#' @export
#' @importFrom stats runif rnorm
seed_sprott_7e <- function(fast_forward, seed = if (!missing(fast_forward)) 1 else NA) {
  length_a <- 14
  if (!is.na(seed)) {
    set.seed(seed)
  }
  if (!missing(fast_forward)) {
    runif(2 * length_a * (fast_forward - 1))
  }
  rnorm(length_a, sd = 1)
}
