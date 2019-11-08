extern crate reqwest;
extern crate regex;

use crate::searcher::Quote;
use regex::Regex;

/// Url of the source to scrap
///
/// Can be changed to scrap another source
static SOURCE: &str = "https://fr.wikiquote.org";

/// Url of the page to scrap
///
/// Can be change to scrap another page
static PAGE: &str = "/wiki/Kaamelott";


/// Get a vector of quotes
///
/// Send a request to wikiquote to get the page content. Each quote will be set into a Quote struct
/// and be stock into a vector.
pub fn get_quotes() -> Vec<Quote> {
    let mut quotes: Vec<Quote> = vec![];
    let mut character: &str = "";

    let main_page_link = get_uri(&PAGE);
    let uri = main_page_link.as_str();

    let html = request_content(uri);
    let string = parse_content(html.as_str()).unwrap().to_string();

    let lines = string.lines();
    for line in lines {
        match extract_character(line) {
            None => (),
            Some(unwrapped) => character = unwrapped
        }

        let external_link = get_external_link(line);

        match external_link {
            None => (),
            Some(external_link) => {
                let link = get_uri(&external_link);
                let uri = link.as_str();
                let html = request_content( uri);

                match parse_content(html.as_str()) {
                    Some(sub_content) => {
                        for line in sub_content.to_string().lines() {
                            let current_quote = extract_quote(line);
                            match current_quote {
                                None => (),
                                Some(quote) => {
                                    quotes.push(Quote::new(quote, character));
                                }
                            }
                        }
                    }
                    None => {}
                };
            }
        }

        let current_quote = extract_quote(line);
        match current_quote {
            None => (),
            Some(quote) => {
                quotes.push(Quote::new(quote, character));
            }
        }
    }

    quotes
}

///
///
///
fn get_uri(page: &&str) -> String {
    format!("{}/{}", SOURCE, page)
}

/// Get a character name from a line
///
/// Use a regex to filter line and capture the character if present
fn extract_character(text: &str) -> Option<&str> {
    lazy_static! {
        static ref CHARACTER_REGEX: Regex = Regex::new(r#"title="w:[\w ]*">(?P<character>[^<]*)</a></span>"#)
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
        static ref QUOTE_REGEX: Regex = Regex::new(r#"<div class="citation">(?P<quote>(.|\n)*)</div>"#)
        .unwrap();
    }

    QUOTE_REGEX.captures(text).and_then(|cap| {
        cap.name("quote").map(|quote| quote.as_str())
    })
}

/// Check for external link
///
/// Use a regex to determine if the
fn get_external_link(text: &str) -> Option<&str> {
    lazy_static! {
        static ref LINK_REGEX: Regex = Regex::new(r#"</i><a href="(?P<link>[^"]*)"#)
        .unwrap();
    }

    LINK_REGEX.captures(text).and_then(|cap| {
        cap.name("link").map(|link| link.as_str())
    })
}


///
///
///
pub fn request_content(uri: &str) -> String {
    reqwest::get(uri).unwrap().text().unwrap()
}

/// Get the content of the page to analyze
///
/// This function get back the HTML code and shrink the size of the output.
/// Parameters: uri => URL used to get back HTML content
pub fn parse_content(html: &str) -> Option<&str> {
    lazy_static! {
        static ref CONTENT_REGEX: Regex =
        Regex::new(r#"<h2><span.*>(?P<content>[\w\s\W\d_]*)(class="extiw"\stitle="m:Accueil"|<!--)"# )
        .unwrap();
        }

    CONTENT_REGEX.captures(html)
        .and_then(|cap| {
            cap.name("content").map(|content| content.as_str())
        })
}

#[cfg(test)]
mod tests {
    use crate::searcher::scraper::{extract_character, extract_quote, get_external_link};

    #[test]
    fn get_back_a_quote() {
        let line = r#""<div class="citation">Test</div>""#;

        let quote = extract_quote(line).unwrap_or("Aucune citation");

        assert_eq!("Test", quote);
    }

    #[test]
    fn get_back_an_external_link() {
        let line = r#""<dl><dd><i>Voir le recueil de citations&#160;: </i><a href="/wiki/Kaamelott/Arthur" title="Kaamelott/Arthur">Arthur</a><i> </i></dd></dl>""#;

        let link = get_external_link(line).unwrap();

        assert_eq!("/wiki/Kaamelott/Arthur", link);
    }

    #[test]
    fn get_back_an_character() {
        let line = r#""<h3><span class="mw-headline" id="Angharad"><a href="https://fr.wikipedia.org/wiki/Personnages_de_Kaamelott#Angharad" class="extiw" title="w:Personnages de Kaamelott">Angharad la servante</a></span><span class="mw-editsection"><span class="mw-editsection-bracket">[</span><a href="/w/index.php?title=Kaamelott&amp;action=edit&amp;section=2" title="Modifier la section : Angharad">modifier</a><span class="mw-editsection-bracket">]</span></span></h3>""#;

        let character = extract_character(line).unwrap_or("Aucune citation");

        assert_eq!("Angharad la servante", character);
    }
}
