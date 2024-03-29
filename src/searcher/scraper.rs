extern crate reqwest;
extern crate regex;

use crate::searcher::Quote;
use regex::Regex;
use htmlescape::decode_html;

/// Url of the source to scrap
///
/// Can be changed to scrap another source
static SOURCE: &str = "https://fr.wikiquote.org";

/// Url of the page to scrap
///
/// Use to scrap the main page
static PAGE: &str = "/wiki/Kaamelott";


/// Get the content of the page asked by parameter.
///
///
fn get_page_content(page: &str) -> Option<String> {
    let uri = get_uri(&page);
    let html = request_content(uri.as_str());

    match parse_content(html.as_str()) {
        None => None,
        Some(rep) => Some(rep.to_string())
    }
}

/// Get a vector of quotes
///
/// Send a request to wikiquote to get the page content. Each quote will be set into a Quote struct
/// and will be stocked into a vector.
pub fn get_quotes() -> Vec<Quote> {
    let mut quotes: Vec<Quote> = vec![];
    let mut character: &str = "";

    let content = get_page_content(&PAGE);

    for line in content.unwrap().lines() {
        match extract_character(line) {
            None => (),
            Some(unwrapped) => character = unwrapped
        }

        match get_external_link(line) {
            None => (),
            Some(external_link) => {
                match get_page_content(&external_link) {
                    Some(sub_content) => {
                        sub_content.to_string().lines().for_each(|line| {
                            match extract_quote(line) {
                                None => (),
                                Some(quote) => {
                                    quotes.push(Quote::new(quote.as_str(), character));
                                }
                            }
                        });
                    }
                    None => {}
                };
            }
        }

        let current_quote = extract_quote(line);
        match current_quote {
            None => (),
            Some(quote) => {
                quotes.push(Quote::new(quote.as_str(), character));
            }
        }
    }

    quotes
}

/// This function permits to get the full URI.
///
/// It's just a concatenation of base URL (set here by constant) and the page to be reached.
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
fn extract_quote(text: &str) -> Option<String> {
    lazy_static! {
        static ref QUOTE_REGEX: Regex = Regex::new(r#"<div class="citation">(?P<quote>(.|\n)*)</div>"#)
        .unwrap();
    }

    QUOTE_REGEX.captures(text).and_then(|cap| {
        cap.name("quote")
            .map(|quote| decode_html(quote.as_str()).unwrap_or_default())
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


/// Send a request to URI provided.
///
/// Returns html content as String object.
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
            Regex::new(r#"<h2><span.*>(?P<content>[\w\s\W\d_]*)<!--"# ).unwrap();
    }

    CONTENT_REGEX.captures(html)
        .and_then(|cap| {
            cap.name("content").map(|content| content.as_str())
        })
}

#[cfg(test)]
mod tests {
    use crate::searcher::scraper::{extract_character, extract_quote, get_external_link, parse_content, get_uri};

    #[test]
    fn get_back_a_quote() {
        let line = r#""<div class="citation">Test</div>""#;

        let quote = extract_quote(line).unwrap_or("Aucune citation".to_string());

        assert_eq!("Test", quote);
    }

    #[test]
    fn get_back_an_uri() {
        let uri = get_uri(&"test");

        assert_eq!("https://fr.wikiquote.org/test", uri);
    }

    #[test]
    fn get_parsed_content() {
        let line1 = r#""<h2><span>test<!--""#;

        let content1 = parse_content(line1).unwrap_or("Aucun contenut");

        assert_eq!("test", content1);
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
