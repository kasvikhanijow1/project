mod modules;

use modules::read_csv;
use modules::normalize;
use modules::linear_regression;
use modules::coefficient_of_determination;


//i used the main function for it to reads the data, normalizes it, performs linear regression, evaluates the model with R², 
//and prints the results according to the data set

fn main() {
    let file_path = "Spotify_final_dataset.csv";

    let (days, total_streams) = match read_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
            return;
        }
    };

    if days.is_empty() || total_streams.is_empty() {
        eprintln!("No valid data to process.");
        return;
    }

  
