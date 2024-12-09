use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};


//this code reads the CSV file and extracts data for days (x) and total streams (y).

pub fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut days = Vec::new();
    let mut total_streams = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue; 
        }
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if cols.len() < 9 {
            days.push(0.0);
            total_streams.push(0.0);
            continue;
        }

        let days_value = cols[3]; 
        let streams_value = cols[8]; 

        let parsed_days = days_value.parse::<f64>().unwrap_or(0.0);
        let parsed_streams = streams_value.parse::<f64>().unwrap_or(0.0);

        days.push(parsed_days);
        total_streams.push(parsed_streams);
    }

    Ok((days, total_streams))
}

//i normalized the data to have mean = 0 and std = 1.

pub fn normalize(data: &Vec<f64>) -> (Vec<f64>, f64, f64) {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let std = (data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64).sqrt();

    let normalized_data = if std > 0.0 {
        data.iter().map(|&x| (x - mean) / std).collect()
    } else {
        data.clone()
    };
    (normalized_data, mean, std)
}

//this runs the linear regression using gradient descent

pub fn linear_regression(
    x: &Vec<f64>,
    y: &Vec<f64>,
    learning_rate: f64,
    iterations: usize,
) -> (f64, f64) {
    let mut slope = 0.0; // m
    let mut intercept = 0.0; // b
    let n = x.len() as f64;

    for _ in 0..iterations {
        let mut slope_gradient = 0.0;
        let mut intercept_gradient = 0.0;

        for (xi, yi) in x.iter().zip(y.iter()) {
            let prediction = slope * xi + intercept;
            slope_gradient += -2.0 * xi * (yi - prediction);
            intercept_gradient += -2.0 * (yi - prediction);
        }

        slope -= learning_rate * (slope_gradient / n);
        intercept -= learning_rate * (intercept_gradient / n);
    }

    (slope, intercept)
}


