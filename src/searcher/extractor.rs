extern crate serde_json;

use crate::searcher::Quote;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{Write, BufReader};

/// Read quote from a file
///
/// Use the path provided as parameter to read file and serialize a vector of Quote.
pub fn read_quote_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Quote>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let quote: Vec<Quote> = serde_json::from_reader(reader)?;

    Ok(quote)
}

/// Write quote into a file
///
/// Based on the path configured, all quotes are serialized and written on a json file.
pub fn write_quote_into_file(quotes: Vec<Quote>, path: &String) -> () {
    let mut output = File::create(path).unwrap();
    let json = serde_json::to_string(&quotes).unwrap();

    match write!(output, "{}", json.as_str()) {
        Ok(_t) => println!("File has been written."),
        Err(e) => println!("File can't be write. {}", e)
    };
}

