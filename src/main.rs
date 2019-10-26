#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod searcher;

use rocket::request::Form;
use rocket::State;
use crate::searcher::Quote;

#[derive(FromForm)]
struct Input {
    text: String
}

#[post("/", data = "<input>")]
fn index(input: Form<Input>, quotes: State<Vec<Quote>>) -> String {

    searcher::search(input.into_inner().text, &quotes)
}

fn main() {
    let quotes = searcher::init();

    rocket::ignite()
        .manage(quotes)
        .mount("/", routes![index])
        .launch();
}
