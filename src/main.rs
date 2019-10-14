#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate quoter;
extern crate rand;

use rocket::request::Form;

use rand::Rng;

#[derive(FromForm)]
struct Input {
    text: String
}

#[post("/", data = "<input>")]
fn index(input: Form<Input>) -> String {
    let mut vec: Vec<String> = vec![
        String::from("Pour attraper des bêtes, il faut imiter les femelles. Là, pour le coup, hop&nbsp;! La femelle lièvre."),
        String::from("Bon, je vais essayer de trouver un petit lièvre pour ce soir parce qu’il commence à faire faim&nbsp;!"),
        String::from("Je me demande comment vous faîtes, moi je serais incapable de trouver de quoi manger dans la forêt.")
    ];

    let pattern: String = input.into_inner().text;
    vec.retain(|x| { x.contains(&pattern) });

    let mut rng = rand::thread_rng();

    let entry_count: usize = vec.len();

    let found: String = match entry_count == 0 {
        false => vec.get(rng.gen_range(0,entry_count)).unwrap().to_string(),
        true => String::from("Aucune citation trouvée"),
    };

    format!("{:?}", found)

}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
