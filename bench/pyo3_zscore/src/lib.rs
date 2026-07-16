use pyo3::prelude::*;
use numpy::{PyArray1, PyReadonlyArray1};

#[pyfunction]
fn zscore<'py>(py: Python<'py>, values: PyReadonlyArray1<f64>) -> &'py PyArray1<f64> {
    let v = values.as_slice().unwrap();
    let n = v.len() as f64;
    let mean = v.iter().sum::<f64>() / n;
    let std = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    let out: Vec<f64> = v.iter().map(|x| (x - mean) / std).collect();
    PyArray1::from_vec(py, out)
}

#[pymodule]
fn pyo3_zscore(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zscore, m)?)?;
    Ok(())
}
