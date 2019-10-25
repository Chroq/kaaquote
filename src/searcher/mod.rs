mod extractor;
pub mod scraper;

extern crate rand;
extern crate serde;
extern crate serde_derive;

use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use rand::Rng;

/// Const PATH : Used to declare the path of file writing
const PATH: &str = "data/kaa.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    text: String,
}

impl Quote {
    /// Format a quote
    ///
    ///
    fn format(&self) -> String {
        format!("{}", self.text)
    }
}


///
///
/// @todo: Put this in memory !
pub fn get() -> Vec<Quote> {
    extractor::read_quote_from_file(PATH).unwrap()
}

///
///
///
pub fn init() -> () {
    if !Path::new(PATH).exists() {
        println!("Récupération des citations");
        let quotes = scraper::get_quotes();

        println!("Écriture local du fichier");
        extractor::write_quote_into_file(quotes);
    }
}

///
///
///
pub fn search(word: String, mut quotes: Vec<Quote>) -> String {
    quotes.retain(|x| { x.text.contains(&word) });

    let mut rng = rand::thread_rng();

    let entry_count: usize = quotes.len();

    match entry_count == 0 {
        false => quotes.get(rng.gen_range(0, entry_count)).unwrap().format(),
        true => String::from("Aucune citation trouvée"),
    }
}