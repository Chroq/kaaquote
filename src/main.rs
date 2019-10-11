#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate quoter;

use rocket::request::Form;
use regex::Regex;

#[derive(FromForm)]
struct Input {
    text: String
}

#[post("/", data = "<input>")]
fn index(input: Form<Input>) -> String {
    let vec: Vec<String> = vec![
        "Bon, je vais essayer de trouver un petit lièvre pour ce soir parce qu’il commence à faire faim&nbsp;!".to_string(),
        "Pour attraper des bêtes, il faut imiter les femelles. Là, pour le coup, hop&nbsp;! La femelle lièvre.".to_string(),
        "Je me demande comment vous faîtes, moi je serais incapable de trouver de quoi manger dans la forêt.".to_string()
    ];

    let pattern: String = input.into_inner().text;
    let re = Regex::new(&pattern).unwrap();

    let found: Vec<String> = vec
        .into_iter()
        .filter(|x| x.contains(&pattern))
        .collect();

    format!("{}", found.first())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
