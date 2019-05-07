test_that("quantile() works", {
  x <- runif(100)
  qs <- c(0, 0.1, 0.9, 1)

  z1 <- unname(quantile(x, qs, type = 1))
  z2 <- quantile_cpp(x, qs)

  expect_equal(z1, z2)
  expect_identical(z1, z2)

})
