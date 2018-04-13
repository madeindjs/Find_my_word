#![feature(plugin)]
#![plugin(rocket_codegen)]
// #[macro_use]
// extern crate serde_derive;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Json;
use std::path::Path;
use rocket::response::NamedFile;


/// Home page
#[get("/")]
pub fn home() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/home.html")).ok()
}

/// JSON API
#[get("/<word>", format = "application/json")]
pub fn query(word: String) -> Option<Json> {
    // todo find word here
    Some(Json(json!({
        "results": [
            "First word",
            "Lorem ipsum",
            word,
        ]
    })))
}


fn main() {
    rocket::ignite()
        .mount("/", routes![home])
        .mount("/query", routes![query])
        .launch();
}
