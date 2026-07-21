pub extern "C" fn doubles(input: *const f64, len: usize, out: *mut f64, out_len: usize) { }
pub extern "C" fn sum_slice(xs: *const f64, len: usize) -> f64 { 0.0 }
pub extern "C" fn fill_slice(out: *mut f64, len: usize, val: f64) { }
