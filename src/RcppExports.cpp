// Generated by using Rcpp::compileAttributes() -> do not edit by hand
// Generator token: 10BE3573-1514-4C36-9D1C-5A225CD40393

#include <Rcpp.h>

using namespace Rcpp;

// discretize_vectors_cpp
NumericMatrix discretize_vectors_cpp(NumericVector x, NumericVector y, NumericVector dims, NumericVector x_range, NumericVector y_range);
RcppExport SEXP _attractor_discretize_vectors_cpp(SEXP xSEXP, SEXP ySEXP, SEXP dimsSEXP, SEXP x_rangeSEXP, SEXP y_rangeSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< NumericVector >::type x(xSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type y(ySEXP);
    Rcpp::traits::input_parameter< NumericVector >::type dims(dimsSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type x_range(x_rangeSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type y_range(y_rangeSEXP);
    rcpp_result_gen = Rcpp::wrap(discretize_vectors_cpp(x, y, dims, x_range, y_range));
    return rcpp_result_gen;
END_RCPP
}
// discretize_cpp
NumericMatrix discretize_cpp(DataFrame data, NumericVector dims, NumericVector x_range, NumericVector y_range);
RcppExport SEXP _attractor_discretize_cpp(SEXP dataSEXP, SEXP dimsSEXP, SEXP x_rangeSEXP, SEXP y_rangeSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< DataFrame >::type data(dataSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type dims(dimsSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type x_range(x_rangeSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type y_range(y_rangeSEXP);
    rcpp_result_gen = Rcpp::wrap(discretize_cpp(data, dims, x_range, y_range));
    return rcpp_result_gen;
END_RCPP
}
// strange_attractor_cpp
List strange_attractor_cpp(NumericVector a, int n, double x0, double y0, bool display_progress);
RcppExport SEXP _attractor_strange_attractor_cpp(SEXP aSEXP, SEXP nSEXP, SEXP x0SEXP, SEXP y0SEXP, SEXP display_progressSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< NumericVector >::type a(aSEXP);
    Rcpp::traits::input_parameter< int >::type n(nSEXP);
    Rcpp::traits::input_parameter< double >::type x0(x0SEXP);
    Rcpp::traits::input_parameter< double >::type y0(y0SEXP);
    Rcpp::traits::input_parameter< bool >::type display_progress(display_progressSEXP);
    rcpp_result_gen = Rcpp::wrap(strange_attractor_cpp(a, n, x0, y0, display_progress));
    return rcpp_result_gen;
END_RCPP
}
// quantile_cpp
NumericVector quantile_cpp(NumericVector x, NumericVector q);
RcppExport SEXP _attractor_quantile_cpp(SEXP xSEXP, SEXP qSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< NumericVector >::type x(xSEXP);
    Rcpp::traits::input_parameter< NumericVector >::type q(qSEXP);
    rcpp_result_gen = Rcpp::wrap(quantile_cpp(x, q));
    return rcpp_result_gen;
END_RCPP
}
// strange_attractor_discretized_cpp
NumericMatrix strange_attractor_discretized_cpp(const NumericVector& a, const int& n, const double& x0, const double& y0, const NumericVector& dims, const NumericVector& qs, const bool display_progress, const int n_discretize);
RcppExport SEXP _attractor_strange_attractor_discretized_cpp(SEXP aSEXP, SEXP nSEXP, SEXP x0SEXP, SEXP y0SEXP, SEXP dimsSEXP, SEXP qsSEXP, SEXP display_progressSEXP, SEXP n_discretizeSEXP) {
BEGIN_RCPP
    Rcpp::RObject rcpp_result_gen;
    Rcpp::RNGScope rcpp_rngScope_gen;
    Rcpp::traits::input_parameter< const NumericVector& >::type a(aSEXP);
    Rcpp::traits::input_parameter< const int& >::type n(nSEXP);
    Rcpp::traits::input_parameter< const double& >::type x0(x0SEXP);
    Rcpp::traits::input_parameter< const double& >::type y0(y0SEXP);
    Rcpp::traits::input_parameter< const NumericVector& >::type dims(dimsSEXP);
    Rcpp::traits::input_parameter< const NumericVector& >::type qs(qsSEXP);
    Rcpp::traits::input_parameter< const bool >::type display_progress(display_progressSEXP);
    Rcpp::traits::input_parameter< const int >::type n_discretize(n_discretizeSEXP);
    rcpp_result_gen = Rcpp::wrap(strange_attractor_discretized_cpp(a, n, x0, y0, dims, qs, display_progress, n_discretize));
    return rcpp_result_gen;
END_RCPP
}

static const R_CallMethodDef CallEntries[] = {
    {"_attractor_discretize_vectors_cpp", (DL_FUNC) &_attractor_discretize_vectors_cpp, 5},
    {"_attractor_discretize_cpp", (DL_FUNC) &_attractor_discretize_cpp, 4},
    {"_attractor_strange_attractor_cpp", (DL_FUNC) &_attractor_strange_attractor_cpp, 5},
    {"_attractor_quantile_cpp", (DL_FUNC) &_attractor_quantile_cpp, 2},
    {"_attractor_strange_attractor_discretized_cpp", (DL_FUNC) &_attractor_strange_attractor_discretized_cpp, 8},
    {NULL, NULL, 0}
};

RcppExport void R_init_attractor(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
