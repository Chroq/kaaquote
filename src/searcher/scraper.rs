extern crate reqwest;

use scraper::{Html, Selector};
use crate::searcher::Quote;

/// Get a vector of quotes
///
/// Send a request to wikiquote to get the page content.
/// In data, will write json file
pub fn get_quotes() -> Vec<Quote> {
    let mut quotes: Vec<Quote> = vec![];

    let document = Html::parse_document(
        reqwest::get("https://fr.wikiquote.org/wiki/Kaamelott")
            .unwrap()
            .text()
            .unwrap()
            .as_str());

    let quote = Selector::parse("div.citation").unwrap();
    for element in document.select(&quote) {
        let text: String = element.inner_html();
        quotes.push(Quote { text });
    }

    quotes
}

