pub fn mean_int_vector_f64(list: &[u128]) -> f64 {
	let sum: u128 = Iterator::sum(list.iter());
	let sum_i32 = sum as i32;
    f64::from(sum_i32) / (list.len() as f64)
}
