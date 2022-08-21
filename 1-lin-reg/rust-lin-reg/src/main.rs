use std::iter::FromIterator;
use ml::models::lin_reg;
use ml::parse;



fn main() {

    let linear_model = &mut lin_reg::LinearRegression{
        coefficient: 0.0,
        intercept: 0.0,
    };

    let data = parse::read_data().unwrap();

    let x_train = Vec::from_iter(data[0][0..50].iter().cloned());
    let y_train = Vec::from_iter(data[1][0..50].iter().cloned());

    let x_test = Vec::from_iter(data[0][50..60].iter().cloned());
    let y_test = Vec::from_iter(data[1][50..60].iter().cloned());

    linear_model.fit(&x_train, &y_train);

    let pred = linear_model.predict(&x_test);
    let score = linear_model.r2(&y_test, &pred);

    println!("R^2 Score: {}", score);
    println!("Coefficient: {}, intercept: {}", linear_model.coefficient, linear_model.intercept)
}