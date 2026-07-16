use std::collections::HashMap;
pub fn count_chars(s: &str) -> HashMap<char, u32> {
	let mut m = HashMap::new();
	for c in s.chars() { *m.entry(c).or_insert(0) += 1; }
	m
}
pub fn pair_sum(pair: (f64, f64)) -> f64 { pair.0 + pair.1 }
