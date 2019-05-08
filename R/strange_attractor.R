#' Generate strange attractor.
#'
#' @param a Numeric vector of length 14
#' @param n Number of points to generate
#' @param x0 Initial value for x
#' @param y0 Initial value for y
#' @param progress If TRUE, displays a progress bar
#'
#' @return Data frame with columns `x` and `y`, and `n` rows.
#' @export
#'
strange_attractor <- function(a, n, x0 = 1, y0 = 1, progress = (n >= 1e6)){
  assert_that(is.vector(a))
  assert_that(all(is.numeric(a)))
  assert_that(length(a) == 14)
  assert_that(is.numeric(n))
  assert_that(length(n) == 1)
  assert_that(n > 0)
  assert_that(is.numeric(x0))
  assert_that(length(x0) == 1)
  assert_that(is.numeric(y0))
  assert_that(length(y0) == 1)

  z <- strange_attractor_cpp(a, n, x0, y0, display_progress = progress)
  class(z) <- c("attractor", "data.frame")
  z
}

#' Generate strange attractor.
#'
#' @inherit strange_attractor
#' @export
#' @importFrom assertthat assert_that
strange_attractor_discretized <- function(a, n, x0 = 1, y0 = 1,
                                          dims = c(600, 600),
                                          progress = (n >= 1e5), n_discretize = 1e6){
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


  assert_that(is.numeric(dims))
  assert_that(length(dims) == 2)

  z <- strange_attractor_discretized_cpp(a, n = n,
                                         x0, y0, dims = dims,
                                         display_progress = progress,
                                         n_discretize = n_discretize)
  z
}



#' @importFrom stats runif rnorm
fast_forward_seed <- function(i, seed = 1){
  set.seed(seed)
  runif(2 * 14 * (i - 1))
  invisible(NULL)
}

#' Generate attractor from specific seed value
#'
#' @inheritParams strange_attractor
#' @param i "Index value" to retrieve
#' @param seed Original seed value, passed to [set.seed()]
#'
#' @export
seeded_attractor <- function(i, n = 1e5, seed = 1){
  fast_forward_seed(i, seed)
  a <- rnorm(14, sd = 1)
  strange_attractor(a, n = n)
}
