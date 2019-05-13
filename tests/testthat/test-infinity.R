test_that("attractor catches infinity values", {
  attractor:::fast_forward_seed(88506)
  a <- rnorm(14)
  z <- a %>%
    strange_attractor(1e5, qs = c(0.05), dims = c(600, 600))
  expect_is(z, "matrix")

})
