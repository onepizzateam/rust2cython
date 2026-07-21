pub extern "C" fn maybe_double(x: f64) -> Option<f64> {
    if x > 0.0 { Some(x * 2.0) } else { None }
}
