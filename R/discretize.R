
#' Discretize two numeric columns of a data frame into a matrix.
#'
#' @param .data Data frame
#' @param dims Dimensions of matrix
#' @param x Name of x column in the data frame
#' @param y Name of y column in the data frame
#'
#' @return A matrix with dimensions `dims`
#' @export
#'
discretize <- function(.data, dims = c(600, 600), x = "x", y = "y"){

  assertthat::assert_that(is.data.frame(.data))
  assertthat::assert_that(is.numeric(.data[[x]]))
  assertthat::assert_that(is.numeric(.data[[y]]))
  assertthat::assert_that(is.numeric(dims))
  assertthat::assert_that(all(dims > 0))
  assertthat::assert_that(length(dims) == 2)
  assertthat::assert_that(nrow(.data) > 0)

  range_x <- range(.data[[x]], na.rm = TRUE)
  range_y <- range(.data[[y]], na.rm = TRUE)

  if (any(is.infinite(c(range_x, range_y)))) {
    warning("Infinite values in .data")
    return(matrix(NA, nrow = dims[1], ncol = dims[2]))
  }
  discretize_cpp(.data[, c(x, y)], dims, range_x, range_y)
}


