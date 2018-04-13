#![feature(plugin)]
#![plugin(rocket_codegen)]
// #[macro_use]
// extern crate serde_derive;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket_contrib::Json;


/// Home page
#[get("/")]
pub fn home() -> Template {
    Template::render("home", &{})
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
        .attach(Template::fairing())
        .launch();
}
