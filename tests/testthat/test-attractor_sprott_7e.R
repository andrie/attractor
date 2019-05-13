context("attractor_sprott_7e")

test_that("strange attractor generates a data frame", {
  n <- 1e3
  a <- c(-0.8, 0.4, -1.1, 0.5, -0.6, -0.1, -0.5, 0.8, 1.0, -0.3, -0.6, -0.3, -1.2, -0.3)
  z <- attractor_sprott_7e(a, n, 1, 1, discretize = FALSE)

  expect_is(z, "data.frame")
  expect_equal(ncol(z), 2)
  expect_equal(names(z), c("x", "y"))
  expect_equal(nrow(z), n)
})


test_that("trim_quantiles() returns a smaller data frame", {
  n <- 1e3
  a <- c(-0.8, 0.4, -1.1, 0.5, -0.6, -0.1, -0.5, 0.8, 1.0, -0.3, -0.6, -0.3, -1.2, -0.3)
  z <- attractor_sprott_7e(a, n, 1, 1, discretize = FALSE)

  q <- trim_quantiles(z)
  expect_is(q, "data.frame")
  expect_true(nrow(q) <= n * 0.9)
})


test_that("recolour() returns a raster", {
  n <- 1e3
  dims <- c(10, 10)
  a <- c(-0.8, 0.4, -1.1, 0.5, -0.6, -0.1, -0.5, 0.8, 1.0, -0.3, -0.6, -0.3, -1.2, -0.3)
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

