#' Title
#'
#' @param x
#' @param palette
#' @param invert
#' @param dims
#' @param zero_colour
#'
#' @return
#' @export
#'
plot.attractor <- function(x, palette = "BuPu", invert = FALSE, dims = c(600, 600), zero_colour = NA, trim_quantiles = 0.02, trans = "log1p"){
  if (any(is.infinite(x[[1]])) || any(is.infinite(x[[2]]))) {
    message("Infinite values")
    return(NULL)
  }

  if (trim_quantiles > 0) {
    x <- trim_quantiles(x, q = trim_quantiles)
  }

  if (nrow(x) == 0) return(invisible(as.raster(matrix(0, nrow = dims[1], ncol = dims[2]))))

  z <-
    x %>%
    discretize(dims = dims) %>%
    match.fun(trans)(.) %>%
    recolour(palette = palette, invert = invert, zero_colour = zero_colour)

  z %>%
    plot()
  invisible(z)
}
