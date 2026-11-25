test_that("discretization works", {
  a <- c(
    -0.8,
    0.4,
    -1.1,
    0.5,
    -0.6,
    -0.1,
    -0.5,
    0.8,
    1.0,
    -0.3,
    -0.6,
    -0.3,
    -1.2,
    -0.3
  )
  n <- 1e4
  dims <- c(10, 10)
  qs <- 0.05

  # The Rust engine bins the serial orbit...
  z2 <- unclass(
    attractor_sprott_7e(a, n, dims = dims, qs = qs, parallel_orbit = 1)
  )
  attr(z2, "class") <- NULL

  # ...and R, given the same orbit and the same percentile range, must agree.
  orbit <- attractor_sprott_7e(a, n, discretize = FALSE)
  rng <- function(v) quantile(v, c(qs, 1 - qs), names = FALSE, type = 7)
  z1 <- bin_xy(orbit$x, orbit$y, dims, rng(orbit$x), rng(orbit$y))

  # The engine's orbit differs from `orbit` by its first and last point only.
  expect_lte(sum(abs(z1 - z2)), 4)
  expect_equal(dim(z2), dims)
})

test_that("the Rust orbit follows the map", {
  a <- seed_sprott_7e(fast_forward = 429)
  o <- attractor_sprott_7e(a, 2000, discretize = FALSE)
  # one step of the map in R, started from each Rust point
  step <- function(x, y) {
    c(
      a[1] + a[2] * x + a[3] * y + a[4] * abs(x)^a[5] + a[6] * abs(y)^a[7],
      a[8] + a[9] * x + a[10] * y + a[11] * abs(x)^a[12] + a[13] * abs(y)^a[14]
    )
  }
  pred <- t(mapply(step, o$x[-2000], o$y[-2000]))
  expect_lt(max(abs(pred[, 1] - o$x[-1]), abs(pred[, 2] - o$y[-1])), 1e-9)
})

test_that("calls reach the compiled Rust library", {
  expect_true("wrap__attractor_matrix" %in% names(getDLLRegisteredRoutines("attractor")$.Call))
  expect_match(paste(deparse(body(attractor:::attractor_matrix)), collapse = " "), ".Call", fixed = TRUE)
})

test_that("short orbits are not padded", {
  a <- seed_sprott_7e(fast_forward = 429)
  z <- attractor_sprott_7e(a, 5e4, dims = c(40, 30), parallel_orbit = 1)
  expect_lte(sum(z), 5e4)
})

test_that("png_sprott_7e writes a PNG", {
  a <- seed_sprott_7e(fast_forward = 429)
  f <- tempfile(fileext = ".png")
  expect_identical(png_sprott_7e(a, f, n = 1e6, size = c(120, 80)), f)
  expect_true(file.exists(f))
  expect_identical(readBin(f, "raw", 8), as.raw(c(0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a)))
  expect_error(png_sprott_7e(a, f, gradient = "nope", size = c(10, 10)))
})
