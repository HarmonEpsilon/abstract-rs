#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use std::io;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct TemplateContext {
    title: String
}

#[get("/")]
fn take_me_home() -> Redirect {
    Redirect::to("/home")
}

#[get("/home")]
fn home() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("home", &context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![take_me_home, home, files])
}

fn main() {
    rocket().launch();
}
