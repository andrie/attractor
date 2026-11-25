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
discretize <- function(.data, dims = c(600, 600), x = "x", y = "y") {
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
  bin_xy(.data[[x]], .data[[y]], dims, range_x, range_y)
}

#' Bin points into a count matrix, truncating, as the Rust engine does.
#'
#' Row 1 holds the largest x. Points outside the ranges are dropped.
#' @noRd
bin_xy <- function(x, y, dims, range_x, range_y) {
  scale_01 <- function(v, r) {
    if (r[1] == r[2]) rep(0.5, length(v)) else (v - r[1]) / (r[2] - r[1])
  }
  rows <- dims[1]
  cols <- dims[2]
  ix <- trunc(scale_01(x, range_x) * (rows - 1))
  iy <- trunc(scale_01(y, range_y) * (cols - 1))
  keep <- !is.na(ix) & !is.na(iy) & ix >= 0 & iy >= 0 & ix < rows & iy < cols
  cell <- 1 + (rows - 1 - ix[keep]) + rows * iy[keep]
  matrix(tabulate(cell, nbins = rows * cols), nrow = rows, ncol = cols)
}
