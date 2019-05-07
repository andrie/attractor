#' Trim x and y range based on quantiles
#'
#' @param x A data frame with columns x and y
#' @param q Quantiles cutoff
#'
#' @return A data frame
#' @export
#'
#' @importFrom stats na.omit quantile
#'
trim_quantiles <- function(x, q = 0.05){
  x_range <- quantile(x$x, probs = c(q, 1 - q), na.rm = TRUE, type = 1)
  y_range <- quantile(x$y, probs = c(q, 1 - q), na.rm = TRUE, type = 1)

  x$x[x$x < x_range[1]] <- NA
  x$x[x$x > x_range[2]] <- NA
  x$y[x$y < y_range[1]] <- NA
  x$y[x$y > y_range[2]] <- NA
  na.omit(x)
}
