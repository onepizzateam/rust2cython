pub fn sum_slice(data: &[f64]) -> f64 {
    data.iter().sum()
}

pub fn byte_count(data: &[u8]) -> usize {
    data.len()
}

pub fn describe(name: &str, values: &[f64]) -> f64 {
    let _ = name;
    values.iter().sum()
}
