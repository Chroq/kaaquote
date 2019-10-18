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
    searcher::search(input.into_inner().text)
}

fn main() {
    println!("{}", searcher::scrapper::write_json_quotes());

    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
