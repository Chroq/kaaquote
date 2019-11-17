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

#[derive(Serialize)]
pub struct QuoteResponse {
    text: String,
    response_type: String,
}

impl QuoteResponse {

    pub fn new(quote: Quote) -> QuoteResponse {
        QuoteResponse {
            text: quote.format(),
            response_type: "in_channel".to_string()
        }
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    text: String,
    character: String,
}

impl Quote {

    /// create a quote based on & str
    fn new(text: &str, character: &str) -> Quote {
        Quote {
            text: text.to_string(),
            character: character.to_string(),
        }
    }

    /// Format a quote struct
    fn format(&self) -> String {
        format!("`{}` _{}_", self.text, self.character)
    }

    /// Search into text the pattern
    fn search_for_word(&self, word: &str) -> bool {
        self.text.contains(&word)
    }
}

///
///
///
pub fn init() -> Vec<Quote> {
    let file_path = format!("{}/{}", DATA_DIRECTORY, FILE);
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
pub fn search(word: String, slice: &[Quote]) -> Quote {
    let mut founds = slice.to_vec();
    founds.retain(|x| x.search_for_word(&word));

    let entry_count = founds.len();

    match entry_count {
        0 => Quote::new("Aucune citation trouvée", ""),
        _ => {
            let position = rand::thread_rng().gen_range(0, entry_count);
            founds.get(position).unwrap().clone()
        }
    }
}