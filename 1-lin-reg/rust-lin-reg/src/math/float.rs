pub fn mean(list: &Vec<f64>) -> f64 {
    let sum: f64 = list.iter().sum();

    sum / (list.len() as f64)
}