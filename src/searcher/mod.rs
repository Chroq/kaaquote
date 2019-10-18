mod extractor;

extern crate rand;
extern crate serde;
extern crate serde_derive;

use serde_derive::{Deserialize, Serialize};

use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    text: String,
    character: String,
}

impl Quote {
    fn format(&self) -> String {
        format!("{} - {}", self.text, self.character)
    }
}

pub fn search(word: String) -> String {
    let mut quotes: Vec<Quote> = extractor::read_quote_from_file("/home/chris/Apps/www/kaaquote/data/sample.json").unwrap();

    quotes.retain(|x| { x.text.contains(&word) });

    let mut rng = rand::thread_rng();

    let entry_count: usize = quotes.len();

    let found: String = match entry_count == 0 {
        false => quotes.get(rng.gen_range(0, entry_count)).unwrap().format(),
        true => String::from("Aucune citation trouvée"),
    };
    found
}



