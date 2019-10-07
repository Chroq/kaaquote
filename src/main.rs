#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate quoter;

use rocket::request::Form;
use quoter::crawler;

#[derive(FromForm)]
struct Input {
    text: String
}

#[post("/", data = "<input>")]
fn index(input: Form<Input>) -> String {

    let word: String = input.into_inner().text;

//    quoter::searcher::search(word);

    format!("{} !", word)
}

fn main() {

    // @todo: Change this to something more parametrable. Env variable ?
    let uri = "https://fr.wikiquote.org/wiki/Kaamelott".parse().unwrap();

    crawler::generate(uri);

    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
