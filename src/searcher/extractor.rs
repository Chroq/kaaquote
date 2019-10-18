extern crate serde_json;

use crate::searcher::Quote;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_quote_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Quote>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let quote: Vec<Quote> = serde_json::from_reader(reader)?;

    Ok(quote)
}


