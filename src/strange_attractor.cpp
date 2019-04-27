#include <Rcpp.h>
using namespace Rcpp;

// [[Rcpp::depends(RcppProgress)]]
#include <progress.hpp>
#include <progress_bar.hpp>

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
  // NumericMatrix xx(4, 5);
  // int xsize = xx.nrow() * xx.ncol();
  // for (int i = 0; i < xsize; i++) {
  //   xx[i] = 7;
  // }
  // return xx;
  NumericVector x(n);
  NumericVector y(n);
  x[0]=x0;
  y[0]=y0;

  Progress p(n, display_progress);


  for(int i = 1; i < n; ++i) {
    // if (Progress::check_abort() )
    //   return -1.0;
    p.increment(); // update progress

    x[i] = a1+a2*x[i-1]+ a3*y[i-1]+ a4*pow(fabs(x[i-1]), a5)+ a6*pow(fabs(y[i-1]), a7);
    y[i] = a8+a9*x[i-1]+ a10*y[i-1]+ a11*pow(fabs(x[i-1]), a12)+ a13*pow(fabs(y[i-1]), a14);
  }
  // return a new data frame
  return DataFrame::create(_["x"]= x, _["y"]= y);

}
