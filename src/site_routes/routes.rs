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

//Redirect to Home 
#[get("/")]
pub fn take_me_home() -> Redirect {
    Redirect::to("/home")
}

//Get request for Home, output Home template
#[get("/home")]
pub fn home() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("home", &context)
}

//Get request for About page, output About template
#[get("/about")]
pub fn about() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("about", &context)
}

//Get request for Register page, output Register template
#[get("/register")]
pub fn sign_up() -> Template {
    let context = TemplateContext {title: "Abstract".to_string()};
    Template::render("register", &context)
}

//Hook up files such as CSS and JavaScript
#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}