/// Dot product of two vectors
#[no_mangle]
pub extern "C" fn dot_product(a: *const f64, a_len: usize,
                               b: *const f64, b_len: usize) -> f64 {
    let a = unsafe { std::slice::from_raw_parts(a, a_len) };
    let b = unsafe { std::slice::from_raw_parts(b, b_len) };
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// L2 norm of a vector
#[no_mangle]
pub extern "C" fn norm(v: *const f64, v_len: usize) -> f64 {
    let v = unsafe { std::slice::from_raw_parts(v, v_len) };
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Scale a vector by a scalar — writes result into out buffer
#[no_mangle]
pub extern "C" fn scale(v: *const f64, v_len: usize,
                         factor: f64,
                         out: *mut f64, out_len: usize) {
    let v = unsafe { std::slice::from_raw_parts(v, v_len) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, out_len) };
    for (o, x) in out.iter_mut().zip(v.iter()) {
        *o = x * factor;
    }
}

#[repr(C)]
pub struct Matrix2x2 {
    pub a: f64, pub b: f64,
    pub c: f64, pub d: f64,
}

/// Determinant of a 2x2 matrix
#[no_mangle]
pub extern "C" fn determinant(m: Matrix2x2) -> f64 {
    m.a * m.d - m.b * m.c
}
