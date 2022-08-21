pub fn vec_add(x: &Vec<f64>, y: f64) -> Vec<f64>{
    let mut output: Vec<f64> = Vec::new();

    for (i, _j) in x.iter().enumerate() {
        output.push(x[i] + y);
    }

    output
}

pub fn vec_multiply(x: &Vec<f64>, y: f64) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();

    for (i, _j) in x.iter().enumerate() {
        output.push(x[i] * y);
    }

    output
}