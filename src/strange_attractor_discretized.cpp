#include <Rcpp.h>
using namespace Rcpp;

// forward declarations

NumericMatrix discretize_vectors_cpp(NumericVector x, NumericVector y, NumericVector dims,
                                     NumericVector x_range, NumericVector y_range);

double scale_01(double x, double min_x, double max_x);

// [[Rcpp::depends(RcppProgress)]]
#include <progress.hpp>
#include <progress_bar.hpp>


// Compute quantiles
// [[Rcpp::export]]
NumericVector quantile_cpp(NumericVector x, NumericVector q) {
  NumericVector y = clone(x);
  std::sort(y.begin(), y.end());
  return y[(y.size() - 1) * q];
}


// [[Rcpp::export]]
NumericMatrix strange_attractor_discretized_cpp(NumericVector a, int n,
                                       double x0, double y0,
                                       NumericVector dims,
                                       bool display_progress=true,
                                       int n_discretize = 1e5) {

  int nd;
  nd = n_discretize;
  if (n < nd) nd = n;

  double a1 = a[0];
  double a2 = a[1];
  double a3 = a[2];
  double a4 = a[3];
  double a5 = a[4];
  double a6 = a[5];
  double a7 = a[6];
  double a8 = a[7];
  double a9 = a[8];
  double a10 = a[9];
  double a11 = a[10];
  double a12 = a[11];
  double a13 = a[12];
  double a14 = a[13];

  NumericVector x(nd);
  NumericVector y(nd);
  x[0] = x0;
  y[0] = y0;

  // Construct empty output matrix
  int rows = dims[0];
  int cols = dims[1];
  NumericMatrix z(rows, cols);
  // std::fill(z.begin(), z.end(), 0);


   // Initialize progress bar
  const int increment_every = 1e5;
  Progress p(ceil(n / increment_every), display_progress);

   // Do the first 100e3 elements

   int i;
   for(i = 0; i < nd - 1; ++i) {

     if (i % increment_every == 0 ) {
       // check for esc key and interrupt calculation
       if (Progress::check_abort()) {
         return z;
       }
       // update progress bar
       p.increment();
     }


     x[i+1] = a1 + a2*x[i] +  a3*y[i] +  a4*pow(fabs(x[i]), a5)  +  a6*pow(fabs(y[i]),  a7);
     y[i+1] = a8 + a9*x[i] + a10*y[i] + a11*pow(fabs(x[i]), a12) + a13*pow(fabs(y[i]), a14);
   }

   // Find quantiles

   NumericVector qs = NumericVector::create(0.02, 0.98);



   // Do initial discretization

   NumericVector x_range;
   NumericVector y_range;
   x_range = quantile_cpp(x, qs);
   y_range = quantile_cpp(y, qs);

   // Rcout << x_range << std::endl;
   // Rcout << y_range << std::endl;

   // Subset x

   LogicalVector index;

   index = (x >= x_range[0]) & (x <= x_range[1]);

   x = x[index];
   y = y[index];

   index = (y >= y_range[0]) & (y <= y_range[1]);

   x = x[index];
   y = y[index];

   z = discretize_vectors_cpp(x, y, dims, x_range, y_range);

   if (n <= nd) return(z);


   // Do the remaining elements

   x[0] = x[i];
   y[0] = y[i];
   i = 0;

   int xx;
   int yy;


   for(int j = nd; j < n - 1; ++j) {

     if (j % increment_every == 0 ) {
       // check for esc key and interrupt calculation
       if (Progress::check_abort()) {
         return z;
       }
       // update progress bar
       p.increment();
     }


     x[i+1] = a1 + a2*x[i] +  a3*y[i] +  a4*pow(fabs(x[i]), a5)  +  a6*pow(fabs(y[i]),  a7);
     y[i+1] = a8 + a9*x[i] + a10*y[i] + a11*pow(fabs(x[i]), a12) + a13*pow(fabs(y[i]), a14);

     xx = round(scale_01(x[i+1], x_range[0], x_range[1]) * (rows - 1));
     yy = round(scale_01(y[i+1], y_range[0], y_range[1]) * (cols - 1));

     // Rcout << j << std::endl;
     // Rcout << xx << std::endl;
     // Rcout << xx << std::endl;

     if ( (xx < rows) & (yy < cols) & (xx >= 0) & ( yy >= 0) ) z(xx, yy) += 1;
     x[0] = x[i+1];
     y[0] = y[i+1];

   }

  // return a matrix
  return z;

}
