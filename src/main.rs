#![feature(plugin)]
#![plugin(rocket_codegen)]
// #[macro_use]
// extern crate serde_derive;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Json;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use rocket::response::NamedFile;


const MAX_WORDS: i8 = 40;

/// Home page
#[get("/")]
fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/home.html")).ok()
}

/// JSON API
#[get("/<word>", format = "application/json")]
fn query(word: String) -> Option<Json> {
    Some(Json(json!({
        "french": get_words("french", &word),
        "english": get_words("english", &word)
    })))
}

fn get_words(lang: &str, patern: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    let mut i = 0i8;

    let file = File::open(format!("dict/{}", lang)).unwrap();
    for line in BufReader::new(file).lines() {

        if i > MAX_WORDS {
            continue;
        }

        match line {
            Ok(line_content) => {
                if line_content.contains(patern) {
                    words.push(str::replace(
                        &line_content,
                        patern,
                        &format!("<span class=\"text-primary\">{}</span>", patern),
                    ));
                    i += 1;
                }
            }
            Err(_) => {}

        };
    }

    return words;

}


fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/query", routes![query])
        .launch();
}
