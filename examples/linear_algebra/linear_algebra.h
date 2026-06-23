#ifndef LINEAR_ALGEBRA_H
#define LINEAR_ALGEBRA_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Matrix2x2 {
    double a;
    double b;
    double c;
    double d;
} Matrix2x2;

double dot_product(const double* a, size_t a_len,
                   const double* b, size_t b_len);

double norm(const double* v, size_t v_len);

void scale(const double* v, size_t v_len,
           double factor,
           double* out, size_t out_len);

double determinant(Matrix2x2 m);

#ifdef __cplusplus
}
#endif

#endif // LINEAR_ALGEBRA_H
