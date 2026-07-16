#[no_mangle]
mod rust_zscore_ffi;

pub fn zscore(values: Vec<f64>) -> Vec<f64> {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let std = (values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    values.iter().map(|x| (x - mean) / std).collect()
}
