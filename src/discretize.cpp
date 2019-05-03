#include <Rcpp.h>
using namespace Rcpp;

double scale_01(double x, double min_x, double max_x){
  if(max_x == min_x) return 0.5;
  return ((x - min_x) / (max_x - min_x));
}


// [[Rcpp::export]]
NumericMatrix discretize_vectors_cpp(NumericVector x, NumericVector y, NumericVector dims,
                                     NumericVector x_range, NumericVector y_range) {
  // Construct empty output matrix
  int rows = dims[0];
  int cols = dims[1];
  NumericMatrix z(rows, cols);
  std::fill(z.begin(), z.end(), 0);

  // Now loop over x and populate z

  unsigned long xsize = x.size();
  for (unsigned long i = 0; i < xsize; i++){
    int xx = round(scale_01(x[i], x_range[0], x_range[1]) * (rows - 1));
    int yy = round(scale_01(y[i], y_range[0], y_range[1]) * (cols - 1));
    if (xx <= rows && yy <= cols) z(xx, yy) += 1;
  }
  return z;
}


// [[Rcpp::export]]
NumericMatrix discretize_cpp(DataFrame data, NumericVector dims,
                             NumericVector x_range, NumericVector y_range) {

  NumericVector x = data[0];
  NumericVector y = data[1];

  return discretize_vectors_cpp(x, y, dims, x_range, y_range);

}


