context("discretize")

test_that("discretization works", {
  dims <- c(10, 10)
  x <- iris
  z <- discretize(x, dims, x = "Sepal.Length", y = "Sepal.Width")

  expect_is(z, "matrix")
  expect_equal(sum(z), nrow(x))
  expect_true(rowSums(z)[1] >= 1)
  expect_true(rowSums(z)[dims[1]] >= 1)
  expect_true(colSums(z)[1] >= 1)
  expect_true(colSums(z)[dims[2]] >= 1)

  x <- data.frame(x = rnorm(1000), y = rnorm(1000))
  z <- discretize(x, dims)

  expect_is(z, "matrix")
  expect_equal(sum(z), nrow(x))
  expect_true(rowSums(z)[1] >= 1)
  expect_true(rowSums(z)[dims[1]] >= 1)
  expect_true(colSums(z)[1] >= 1)
  expect_true(colSums(z)[dims[2]] >= 1)
})
