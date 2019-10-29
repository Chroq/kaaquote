#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod searcher;

use rocket::request::Form;
use rocket::State;
use crate::searcher::Quote;

#[derive(FromForm)]
struct Command {
    text: String,
    token: String,
    team_id: String,
    team_domain: String,
    enterprise_id: String,
    enterprise_name: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    response_url: String,
    trigger_id: String,
}

#[post("/", data = "<input>")]
fn index(input: Form<Command>, quotes: State<Vec<Quote>>) -> String {
    searcher::search(input.into_inner().text, &quotes)
}

fn main() {
    let quotes = searcher::init();

    rocket::ignite()
        .manage(quotes)
        .mount("/", routes![index])
        .launch();
}
