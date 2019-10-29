extern crate reqwest;
extern crate regex;

use crate::searcher::Quote;
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashSet;
use regex::Captures;
use scraper::html::Select;
use std::error::Error;


/// Get a vector of quotes
///
/// Send a request to wikiquote to get the page content. Each quote will be set into a Quote struct
/// and be stock into a vector.
pub fn get_quotes() -> Vec<Quote> {
    let mut quotes: Vec<Quote> = vec![];

    let content: String = get_content();

    let mut character = "";

    for line in content.lines() {
        let current_character = extract_character(line);

        match current_character {
            None => (),
            Some(t) => character = current_character.unwrap()
        }

        let current_quote = extract_quote(line);
        match current_quote {
            None => (),
            Some(t) => {
                quotes.push(Quote {
                    text: current_quote.unwrap().to_string(),
                    character: character.to_string(),
                });
            }
        }
    }

    quotes
}

/// Get a character name from a line
///
/// Use a regex to filter line and capture the character if present
fn extract_character(text: &str) -> Option<&str> {
    lazy_static! {
        static ref CHARACTER_REGEX : Regex = Regex::new(r#"title="w:[\w ]*">(?P<character>[^<]*)</a></span>"#)
            .unwrap();
    }

    CHARACTER_REGEX.captures(text).and_then(|cap| {
        cap.name("character").map(|character| character.as_str())
    })
}

/// Get a quote from a line
///
/// Use a regex to filter line and capture the quote if present
fn extract_quote(text: &str) -> Option<&str> {
    lazy_static! {
        static ref QUOTE_REGEX : Regex = Regex::new(r#"<div class="citation">(?P<quote>(.|\n)*)</div>"#)
            .unwrap();
    }

    QUOTE_REGEX.captures(text).and_then(|cap| {
        cap.name("quote").map(|quote| quote.as_str())
    })
}

/// Get the content of the page to analyze
///
/// This function get back the HTML code and shrink the size of the output.
pub fn get_content() -> String {
    static URL: &str = "https://fr.wikiquote.org/wiki/Kaamelott";

    lazy_static! {
        static ref CONTENT_REGEX : Regex = Regex::new(r"<h2><span.*>(?P<content>[\w\s\W\d_]*)<h2><span\sclass=")
            .unwrap();
    }

    let document = reqwest::get(URL).unwrap().text().unwrap();
    let full_content = document.as_str();

    CONTENT_REGEX.captures(&full_content).and_then(|cap| {
        cap.name("content").map(|content| content.as_str().to_string())
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::searcher::scraper::{extract_character, extract_quote};

    #[test]
    fn get_back_quote() {
        let line = r#""<div class="citation">Test</div>""#;

        let quote = extract_quote(line).unwrap_or("Aucune citation");

        assert_eq!("Test", quote);
    }

    #[test]
    fn get_back_character() {
        let line = r#""<h3><span class="mw-headline" id="Angharad"><a href="https://fr.wikipedia.org/wiki/Personnages_de_Kaamelott#Angharad" class="extiw" title="w:Personnages de Kaamelott">Angharad la servante</a></span><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Kaamelott&amp;action=edit&amp;section=2" title="Modifier la section : Angharad">modifier</a><span class="mw-editsection-bracket">]</span></span></h3>""#;

        let character = extract_character(line).unwrap_or("Aucune citation");

        assert_eq!("Angharad la servante", character);
    }
}
