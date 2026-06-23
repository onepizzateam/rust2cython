/// Dot product of two vectors (must be same length)
pub fn dot_product(a: Vec<f64>, b: Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Scale a vector by a scalar
pub fn scale(v: Vec<f64>, factor: f64) -> Vec<f64> {
    v.iter().map(|x| x * factor).collect()
}

/// L2 norm of a vector
pub fn norm(v: Vec<f64>) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

#[repr(C)]
pub struct Matrix2x2 { pub a: f64, pub b: f64, pub c: f64, pub d: f64 }

pub fn determinant(m: Matrix2x2) -> f64 {
    m.a * m.d - m.b * m.c
}

mod linear_algebra_ffi;
