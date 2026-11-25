#' Draw a Sprott 7e attractor straight to a PNG file.
#'
#' Orbit, binning, colouring and PNG encoding all run in Rust, which makes this
#' the fast route to large images, for example for print. The PNG uses fast
#' compression with no row filter, which trades a larger file for speed.
#'
#' @inheritParams attractor_sprott_7e
#' @param filename Output file name.
#' @param size Image size in pixels, `c(width, height)`.
#' @param gradient Gradient name: one of `"turbo"`, `"viridis"`, `"magma"`,
#'   `"inferno"`, `"plasma"`, `"cividis"`, or a ColorBrewer name such as
#'   `"BuPu"`, `"Oranges"`, `"Reds"`, `"Greens"`, `"Blues"`, `"BuGn"`,
#'   `"Spectral"` or `"PuOr"`.
#'
#' @return The file name, invisibly.
#' @export
#' @family sprott functions
#'
#' @examples
#' a <- seed_sprott_7e(fast_forward = 429)
#' f <- png_sprott_7e(a, tempfile(fileext = ".png"), n = 1e6, size = c(400, 300))
png_sprott_7e <- function(
  a,
  filename = "attractor.png",
  n = 1e7,
  x0 = 1,
  y0 = 1,
  size = c(3840, 2160),
  gradient = "turbo",
  qs = 0.075,
  parallel_orbit = default_parallel_orbit()
) {
  assert_that(is.numeric(a), length(a) == 14)
  assert_that(is.numeric(n), length(n) == 1, n > 0)
  assert_that(is.numeric(size), length(size) == 2, all(size >= 1))
  assert_that(is.character(gradient), length(gradient) == 1)
  assert_that(is.character(filename), length(filename) == 1)
  assert_that(is.numeric(qs), length(qs) == 1, qs >= 0, qs < 0.5)
  assert_that(is.numeric(parallel_orbit), length(parallel_orbit) == 1, parallel_orbit >= 1)
  invisible(attractor_png(
    a,
    n = n,
    x0 = x0,
    y0 = y0,
    size = size,
    perc_threshold = 1 - 2 * qs,
    parallel_orbit = parallel_orbit,
    gradient = gradient,
    filename = filename
  ))
}
