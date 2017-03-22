use rocket::Response;
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
pub fn take_me_home() -> Redirect {
    Redirect::to("/home")
}

#[get("/home")]
pub fn home() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("home", &context)
}

#[get("/about")]
pub fn about() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("about", &context)
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}