test_that("attractor catches infinity values", {
  z <-
    seed_sprott_7e(fast_forward = 88506) %>%
    attractor_sprott_7e(1e5, qs = c(0.05), dims = c(600, 600), progress = FALSE)

    expect_is(z, "matrix")

})
