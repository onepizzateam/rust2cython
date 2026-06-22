pub fn maybe_sqrt(x: f64) -> Option<f64> { if x >= 0.0 { Some(x.sqrt()) } else { None } }
pub fn safe_div(a: i32, b: i32) -> Result<i32, String> { if b == 0 { Err("divide by zero".into()) } else { Ok(a / b) } }
pub fn sum_vec(values: Vec<f64>) -> f64 { values.iter().sum() }
pub fn clamp(x: Option<f64>) -> f64 { x.unwrap_or(0.0) }
