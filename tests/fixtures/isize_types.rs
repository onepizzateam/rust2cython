pub fn signed_index(arr: Vec<f64>, idx: isize) -> f64 {
	let i = if idx < 0 { arr.len() as isize + idx } else { idx } as usize;
	arr[i]
}

pub fn offset_sum(values: Vec<f64>, offset: isize) -> f64 {
	values.iter().sum::<f64>() + offset as f64
}
