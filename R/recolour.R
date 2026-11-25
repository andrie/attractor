#' Flip image by 180 degrees
#'
#' @param x Matrix
#'
#' @return Matrix
#' @keywords Internal
flip_180 <- function(x){
  x[rev(seq_len(nrow(x))), ]
}

# -------------------------------------------------------------------------


#' Invert colours
#'
#' @param x Matrix
#'
#' @return Matrix
#' @keywords Internal
invert <- function(x) {
  1 - x / max(x, na.rm = TRUE)
}

# -------------------------------------------------------------------------


#' Use palette from ColorBrewer to recolour the image.
#'
#' You can use any of the colours defined by the [scales::col_numeric()]
#' function.
#'
#' The palette argument can be any of the following:
#'
#' 1. A character vector of RGB or named colours. Examples: `palette()`,
#' `c("#000000", "#0000FF", "#FFFFFF")`, `topo.colors(10)` 2. The name of an
#' `RColorBrewer` palette, e.g. `"BuPu"` or `"Greens"`. 3. A function that
#' receives a single value between 0 and 1 and returns a colour. Examples:
#' `colorRamp(c("#000000", "#FFFFFF"), interpolate="spline")`.
#'
#' @param x Matrix
#' @param palette ColorBrewer palette. This value is passed to [scales::col_numeric()]
#' @param zero_colour Colour to use for zero and NA values
#' @param invert If `TRUE`, inverts the colour scale
#' @param trans Transformation function, defaulting to [log1p()]
#'
#' @return Matrix
#' @export
#' @importFrom scales col_numeric
#' @importFrom grDevices col2rgb as.raster rgb
#'
#' @example inst/examples/example_recolour.R
#'
recolour <- function(x, palette = "Blues", zero_colour = NA, invert = FALSE, trans = "log1p"){
  class(x) <- "matrix"
  zero_colour <- do.call(rgb, as.list(col2rgb(zero_colour) / 255))
  if (!is.null(trans)) {
    x <- match.fun(trans)(x)
  }
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
