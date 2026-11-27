# context("attractor_sprott_7e")

test_that("strange attractor generates a data frame", {
  n <- 1e3
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
  z <- attractor_sprott_7e(a, n, 1, 1, discretize = FALSE)

  expect_is(z, "data.frame")
  expect_equal(ncol(z), 2)
  expect_equal(names(z), c("x", "y"))
  expect_equal(nrow(z), n)
})


test_that("trim_quantiles() returns a smaller data frame", {
  n <- 1e3
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
  z <- attractor_sprott_7e(a, n, 1, 1, discretize = FALSE)

  q <- trim_quantiles(z)
  expect_is(q, "data.frame")
  expect_true(nrow(q) <= n * 0.9)
})


test_that("recolour() returns a raster", {
  n <- 1e3
  dims <- c(10, 10)
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
  x <- attractor_sprott_7e(a, n, 1, 1, discretize = FALSE)
  y <- discretize(x, dims)

  f <- flip_180(y)
  expect_is(f, "matrix")

  r <- recolor(flip_180(y))
  expect_is(r, "raster")

  r <- recolor(y)
  expect_is(r, "raster")

  r <- recolor(y, invert = TRUE)
  expect_is(r, "raster")
})


test_that("the matrix carries its frame, the same at any size", {
  a <- seed_sprott_7e(2981)
  m1 <- attractor_sprott_7e(a, n = 2e6, dims = c(40, 30), qs = 0.03, parallel_orbit = 4)
  m2 <- attractor_sprott_7e(a, n = 2e6, dims = c(97, 53), qs = 0.03, parallel_orbit = 4)
  for (r in c("xrange", "yrange")) {
    expect_length(attr(m1, r), 2)
    expect_true(all(is.finite(attr(m1, r))))
    expect_lt(attr(m1, r)[1], attr(m1, r)[2])
    expect_identical(attr(m1, r), attr(m2, r))
  }
  # zooming out widens the frame
  m3 <- attractor_sprott_7e(a, n = 2e6, dims = c(40, 30), qs = 0.01, parallel_orbit = 4)
  expect_gt(diff(attr(m3, "xrange")), diff(attr(m1, "xrange")))
})
