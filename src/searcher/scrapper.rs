extern crate hyper;

use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

/// Write JSON data from a website
///
/// Send a request to wikiquote to get the page content.
pub fn write_json_quotes() -> String {

    rt::run(rt::lazy(|| {
        let client = Client::new();

        let uri = "https://fr.wikiquote.org/wiki/Kaamelott".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));

}


