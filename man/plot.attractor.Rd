% Generated by roxygen2: do not edit by hand
% Please edit documentation in R/plot.R
\name{plot.attractor}
\alias{plot.attractor}
\title{Plot a strange attractor.}
\usage{
\method{plot}{attractor}(x, palette = "BuPu", invert = FALSE,
  dims = c(600, 600), zero_colour = NA, trim_quantiles = 0,
  trans = "log1p", ...)
}
\arguments{
\item{x}{Attractor, generated by \code{\link[=attractor_sprott_7e]{attractor_sprott_7e()}}}

\item{palette}{ColorBrewer palette. This value is passed to \code{\link[scales:col_numeric]{scales::col_numeric()}}}

\item{invert}{If \code{TRUE}, inverts the colour scale}

\item{dims}{Dimensions of matrix}

\item{zero_colour}{Colour to use for zero and NA values}

\item{trim_quantiles}{Numeric value, indicating a trim value. Quantiles below this value, as well as higher than \code{(1 - trim_quantiles} will be removed from data prior to plotting.}

\item{trans}{Transforming function}

\item{...}{Other options passed to \code{\link[=plot]{plot()}}}
}
\value{
Raster, invisibly
}
\description{
Plot a strange attractor.
}
\concept{attractor}
