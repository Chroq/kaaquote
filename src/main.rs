#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod searcher;

use rocket::request::Form;

#[derive(FromForm)]
struct Input {
    text: String
}

#[post("/", data = "<input>")]
fn index(input: Form<Input>) -> String {
    searcher::search(input.into_inner().text, searcher::get())
}

fn main() {
    searcher::init();

    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
