test_that("an orbit that diverges is an error, not a matrix", {
  # x -> 10 x and y -> 10 y overflow a double after about 300 steps, whatever
  # the last bits of exp() and ln() are
  a <- c(0, 10, 0, 0, 1, 0, 1, 0, 0, 10, 0, 1, 0, 1)

  expect_error(
    attractor_sprott_7e(a, 1e5, qs = 0.05, dims = c(600, 600), parallel_orbit = 1),
    "diverged"
  )
  expect_error(
    attractor_sprott_7e(a, 1e5, dims = c(600, 600)),
    "diverged"
  )
  expect_error(
    attractor_sprott_7e(a, 1e5, discretize = FALSE),
    "diverged"
  )
  expect_error(png_sprott_7e(a, tempfile(fileext = ".png"), n = 1e5), "diverged")
})
