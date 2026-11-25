# Generate the seed vector (14 random values)
a <- seed_sprott_7e(fast_forward = 429)

# Generate points, trim at the 0.03 quantile, and discretize to a 600x900 matrix
dat <- attractor_sprott_7e(a, n = 1e6, qs = 0.03, dims = c(600, 900))

# Plot using various palettes from `RColorBrewer`
plot(dat, palette = "Oranges")
plot(dat, palette = "Greens")

plot(dat, palette = "Oranges", zero_colour = "#FFFAF6")
plot(dat, palette = "Oranges", zero_colour = "grey20", invert = TRUE)
