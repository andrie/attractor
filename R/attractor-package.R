#' Create and Plot Strange Attractors
#'
#' Julien C. Sprott's book *Strange Attractors: Creating Patterns in Chaos*
#' (1993) shows how to make an endless variety of fractal patterns from simple
#' equations. You don't pick the coefficients yourself. The computer picks them
#' at random, iterates the equations, and keeps the result only if the orbit
#' settles into something worth looking at. The book describes over 350 such
#' patterns, in two, three and more dimensions.
#'
#' This package implements only the example in chapter 7e. Give it a seed and it
#' finds the coefficients for that seed, so the same seed always gives you the
#' same picture. Then it iterates the map for as many points as you ask for.
#'
#' The package is fast, because the iteration runs in `Rust`, and the points are
#' binned (discretised) into a much smaller matrix, so plotting is cheap too.
#'
#' Get started:
#' * [attractor_sprott_7e()]
#'
#' @references
#' Sprott, J. C. (1993). *Strange Attractors: Creating Patterns in Chaos*. M&T
#' Books, New York. Free PDF at <https://sprott.physics.wisc.edu/fractals.htm>.
#'
#' @keywords internal
"_PACKAGE"

# The following block is used by usethis to automatically manage
# roxygen namespace tags. Modify with care!
## usethis namespace: start
## usethis namespace: end
NULL
