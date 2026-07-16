pub fn parse_int(s: &str) -> Result<i32, String> {
	s.parse::<i32>().map_err(|e| e.to_string())
}

pub fn format_float(x: f64, precision: u32) -> Result<String, String> {
	if x.is_nan() { return Err("NaN".into()); }
	Ok(format!("{:.prec$}", x, prec = precision as usize))
}
