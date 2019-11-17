#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket_contrib;

mod searcher;

use crate::searcher::{Quote, QuoteResponse};
use rocket::State;
use rocket::request::LenientForm;
use rocket_contrib::json::JsonValue;

#[derive(FromForm)]
struct Command {
    text: String,
}

#[post("/", data = "<input>")]
fn index(input: LenientForm<Command>, quotes: State<Vec<Quote>>) -> JsonValue {
    let quote = searcher::search(input.into_inner().text, &quotes);

    json!(QuoteResponse::new(quote))
}

fn main() {
    let quotes = searcher::init();

    rocket::ignite()
        .manage(quotes)
        .mount("/", routes![index])
        .launch();
}
