#' Flip image by 180 degrees
#'
#' @param x Matrix
#'
#' @return Matrix
#' @export
flip_180 <- function(x){
  x[rev(seq_len(nrow(x))), ]
}

# -------------------------------------------------------------------------


#' Invert colours
#'
#' @param x Matrix
#'
#' @return Matrix
#' @export
invert <- function(x) {
  1 - x / max(x, na.rm = TRUE)
}

# -------------------------------------------------------------------------


#' Use palette from ColorBrewer to recolour the image.
#'
#' @param x Matrix
#' @param palette ColorBrewer palette. This value is passed to [scales::col_numeric()]
#' @param zero_colour Colour to use for zero and NA values
#' @param invert If `TRUE`, inverts the colour scale
#'
#' @return Matrix
#' @export
#' @importFrom scales col_numeric
#' @importFrom grDevices as.raster
recolour <- function(x, palette = "Blues", zero_colour = NA, invert = FALSE){
  zero_colour <- do.call(rgb, as.list(col2rgb(zero_colour)/255))
  dims <- dim(x)
  if (!missing(zero_colour) && !is.na(zero_colour) && !is.null(zero_colour)) {
    x[x == 0] <- NA
  }
  if (invert) x  <- invert(x)
  col_range <- range(x, na.rm = TRUE)
  col_custom <- scales::col_numeric(palette = palette, domain = col_range, na.color = zero_colour)
  as.raster(
    matrix(
      col_custom(x),
      nrow = dims[1], ncol = dims[2])
  )
}


# -------------------------------------------------------------------------


#' @export
#' @rdname recolour
recolor <- recolour
