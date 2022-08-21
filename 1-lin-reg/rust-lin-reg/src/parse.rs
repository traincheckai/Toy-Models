use std::error::Error;
use std::io;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    X: f64,
    Y: f64,
}

pub fn read_data() -> Result<[Vec<f64>; 2], Box<dyn Error>> {
    let mut X_data: Vec<f64> = Vec::new();
    let mut Y_data: Vec<f64> = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.deserialize() {
        let record: Record = result?;
        X_data.push(record.X);
        Y_data.push(record.Y);
    }

    let data: [Vec<f64>; 2] = [X_data, Y_data];

    Ok(data)
}