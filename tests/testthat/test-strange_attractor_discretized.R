test_that("multiplication works", {

  a <- c(-0.8, 0.4, -1.1, 0.5, -0.6, -0.1, -0.5, 0.8, 1.0, -0.3, -0.6, -0.3, -1.2, -0.3)
  n <- 1e4
  dims = c(10, 10)

  z1 <-
    a %>%
    strange_attractor(n) %>%
    trim_quantiles(q = 0.05) %>%
    discretize(dims)

  z2 <-
    a %>%
    strange_attractor_discretized(n, dims = dims, n_discretize = 1e6)

  expect_true(
    all.equal(z1, z2, tolerance = 1e-2)
  )
})
