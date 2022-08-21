use num::pow;
use crate::math;

pub struct LinearRegression{
    pub coefficient: f64,
    pub intercept: f64,
}

impl LinearRegression{

    pub fn fit(&mut self, x: &Vec<f64>, y: &Vec<f64>){
        self.coefficient = self.estimate_coefficient(x, y);
        self.intercept = self.compute_intercept(x, y);
    }

    pub fn predict(&self, x: &Vec<f64>) -> Vec<f64> {
        let x_times_coef = math::vec::vec_multiply(x, self.coefficient);
        math::vec::vec_add(&x_times_coef, self.intercept)
    }

    pub fn r2(&self, y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64{
        let y_avg = math::float::mean(y_true);
        let mut residual_number_of_squares: f64 = 0.0;
        let mut total_number_of_squares: f64 = 0.0;

        for (i, _j) in y_true.iter().enumerate() {
            residual_number_of_squares = residual_number_of_squares + pow(y_true[i] - y_pred[i], 2);
            total_number_of_squares = total_number_of_squares + pow(y_true[i] - y_avg, 2);
        }

        1.0 - (residual_number_of_squares / total_number_of_squares)
    }

    pub fn compute_intercept(&self, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let x_avg = math::float::mean(x);
        let mul = self.coefficient * x_avg;

        math::float::mean(y) - mul
    }
    
    pub fn estimate_coefficient(&self, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let mut numerator: f64 = 0.0;
        let mut denominator: f64 = 0.0;

        for (i, _j) in x.iter().enumerate(){
            let x_val = x[i];
            let y_val = y[i];
            let x_avg = math::float::mean(x);
            let y_avg = math::float::mean(y);

            numerator = numerator + ((x_val - x_avg) * (y_val - y_avg));
            denominator = denominator + pow(x_val - x_avg, 2);
        }

        numerator / denominator
    }
}
