use std::fs::File;
use std::io::Write;
use reqwest::blocking::get;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&response)?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://github.com/datablist/sample-csv-files/raw/main/files/offers/offers-1000.csv";
    let file_path = "../data/offer.csv";
    extract(url, file_path)?;
    Ok(())
}