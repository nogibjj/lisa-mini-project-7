//use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rand::SeedableRng;
use rusqlite::{params, Connection};
//use std::fs;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed = 42;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let dataset = "../data/offer.csv";

    let file = File::open(dataset)?;
    let mut reader = BufReader::new(file);
    

    let mut conn = Connection::open("OfferDB.db")?;
    conn.execute("DROP TABLE IF EXISTS OfferDB", [])?;
    conn.execute(
        "CREATE TABLE OfferDB (iindex INTEGER, ean TEXT, stock INTEGER, price INTEGER)",
        [],
    )?;
    
    let mut stmt = conn.prepare("INSERT INTO OfferDB VALUES (?, ?, ?, ?)")?;

    let mut buffer = String::new();

    // Read and discard the first line
    let _ = reader.read_line(&mut buffer)?;
    buffer.clear();

    let mut buffer = String::new();
    for line in reader.lines() {
        let line = line?; 
        let row: Vec<&str> = line.split(',').collect();
        let iindex: i32 = match row[0].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid integer for index: {} end", row[0]);
                continue;
            }
        };
        let ean = row[1];
        let stock: i32 = match row[2].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid integer for stock: {} end", row[2]);
                continue;
            }
        };
        let price: i32 = match row[3].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid integer for price: {} end", row[3]);
                continue;
            }
        };

        stmt.execute(params![iindex, ean, stock, price]);
        //printing inserstion code to debug
        //println!("Inserting: {}, {}, {}, {}", iindex, ean, stock, price);
        // match stmt.execute(params![iindex, ean, stock, price]) {
        //     Ok(_) => println!("Insert successful"),
        //     Err(e) => println!("Error inserting: {:?}", e),
        // }
        buffer.clear();
        
    }

    Ok(())
}



        //let iindex: i32 = rng.gen(); // random number generator
    
    
    // below is the code to get the size of the file, to debug and make sure the data is loaded correctly
    // let metadata = fs::metadata(dataset)?;
    // let file_size = metadata.len();
    // println!("The size of the file is {} bytes", file_size);

        //while reader.read_line(&mut buffer)? > 0 {


// use std::error::Error;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;
// use csv;

// pub fn write_to_csv_file(data: &Vec<rusqlite::Row>, filename: &str) -> Result<(), Box<dyn Error>> {
//     let mut wtr = csv::Writer::from_path(filename)?;
//     for row in data {
//         wtr.write_record(row.iter().map(|col| col.to_string()))?;
//     }
//     wtr.flush()?;
//     Ok(())
// }

// pub fn load_and_split(dataset: &str, train_ratio: f32, seed: u64) -> Result<(String, String), Box<dyn Error>> {
  
// }

// pub fn run() -> Result<(String, String), Box<dyn Error>> {
//     let dataset_path = "data/offer.csv";
//     let train_ratio = 0.8;
//     let seed = 42;
//     load_and_split(dataset_path, train_ratio, seed)
// }

// // Load and split the data, using the random seed 42
// //let (train_data_file, test_data_file) = run().unwrap();