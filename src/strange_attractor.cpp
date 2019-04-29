#include <Rcpp.h>
using namespace Rcpp;

// [[Rcpp::depends(RcppProgress)]]
#include <progress.hpp>
#include <progress_bar.hpp>

// Compute quantiles
NumericVector quantile(NumericVector x, NumericVector q) {
  std::sort(x.begin(), x.end());
  return x[x.size() * q];
}

// [[Rcpp::export]]
List strange_attractor_cpp(NumericVector a, int n, double x0, double y0, bool display_progress=true) {
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

  NumericVector x(n);
  NumericVector y(n);
  x[0] = x0;
  y[0] = y0;

  // Initialize progress bar
  const int increment_every = 1e5;
  Progress p(ceil(n / increment_every), display_progress);

  for(int i = 0; i < n - 1; ++i) {

    if (i % increment_every == 0 ) {
      // check for esc key and interrupt calculation
      if (Progress::check_abort()) {
        return DataFrame::create(_["x"]= x, _["y"]= y);
      }
      // update progress bar
      p.increment();
    }


    x[i+1] = a1 + a2*x[i] +  a3*y[i] +  a4*pow(fabs(x[i]), a5)  +  a6*pow(fabs(y[i]),  a7);
    y[i+1] = a8 + a9*x[i] + a10*y[i] + a11*pow(fabs(x[i]), a12) + a13*pow(fabs(y[i]), a14);
  }
  // return a new data frame
  return DataFrame::create(_["x"]= x, _["y"]= y);

}
