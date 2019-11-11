#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod searcher;

use crate::searcher::Quote;
use rocket::State;
use rocket::request::LenientForm;

#[derive(FromForm)]
struct Command {
    text: String,
}

#[post("/", data = "<input>")]
fn index(input: LenientForm<Command>, quotes: State<Vec<Quote>>) -> String {
    searcher::search(input.into_inner().text, &quotes)
}

fn main() {
    let quotes = searcher::init();

    rocket::ignite()
        .manage(quotes)
        .mount("/", routes![index])
        .launch();
}
