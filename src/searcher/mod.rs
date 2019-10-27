mod extractor;
pub mod scraper;

extern crate rand;
extern crate serde;
extern crate serde_derive;

use std::fs;
use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use rand::Rng;

/// Const PATH : Used to declare the path of file writing
const DATA_DIRECTORY: &str = "data";
const FILE: &str = "kaa.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    text: String,
}

impl Quote {
    /// Format a quote struct
    fn format(&self) -> String {
        format!("{}", self.text)
    }
}

///
///
///
pub fn init() -> Vec<Quote> {
    let file_path = format!("{}/{}", DATA_DIRECTORY, FILE );
    if !Path::new(file_path.as_str()).exists() {
        let quotes = scraper::get_quotes();

        match fs::create_dir(DATA_DIRECTORY) {
            Ok(_t) => {}
            Err(_e) => {}
        };
        extractor::write_quote_into_file(quotes, &file_path);
    }

    extractor::read_quote_from_file(file_path).unwrap()
}

///
///
///
pub fn search(word: String, slice: &[Quote]) -> String {
    let mut founds = vec![];

    for quote in slice.iter() {
        if quote.text.contains(&word) {
            founds.push(quote)
        }
    }

    let mut rng = rand::thread_rng();

    let entry_count: usize = founds.len();

    match entry_count == 0 {
        false => founds.get(rng.gen_range(0, entry_count)).unwrap().format(),
        true => String::from("Aucune citation trouvée"),
    }
}