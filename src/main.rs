#![feature(plugin)]
#![plugin(rocket_codegen)]
// #[macro_use]
// extern crate serde_derive;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Template;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

use rocket_contrib::Json;

/*
#[derive(Serialize, Deserialize)]
struct Word {
    title: String,
    description: String,
}

impl Word {
    fn new(title: &str, description: &str) -> Word {
        Word {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}
*/

/// Home page
#[get("/")]
pub fn home() -> Template {
    Template::render("pages/home", &{})
}

/// JSON API
#[get("/<word>", format = "application/json")]
pub fn query(word: String) -> Option<Json> {
    // todo find word here
    Some(Json(json!({
        "results": [
            "First word",
            "Lorem ipsum",
            word
        ]
    })))
}

/// Static files
#[get("/assets/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![home, files])
        .mount("/query", routes![query])
        .attach(Template::fairing())
        .launch();
}
