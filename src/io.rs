use std::fs;
use csv;

#[allow(dead_code)]
pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.split("\n").map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn read_from_csv(path: &str, delimiter: u8) -> Vec<Vec<String>> {
    // Creates a new csv 'Reader' from a file
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .from_path(path).expect("Failed to open CSV!");
    // Retrieve and print header record
    match reader.headers(){
        Ok(header) => {
            println!("Found header: {:?}", header);
        }
        Err(_) => {
            println!("No Header found...")
        }
    }
    let mut contents = vec![];
    for result in reader.records() {
        let record = result.unwrap();
        //println!("{:?}", record);
        let mut row = vec![];
        for cell in &record {
             // match cell.parse::<f64>() {
             //     Ok(value) => {
             //         row.push(value);
             //     }
             //     Err(err) => {
             //         println!("Error {err} parsing value {row:?}");
             //     }
             // }
            row.push(cell.to_string());
        }
        contents.push(row);
    }
    contents
}