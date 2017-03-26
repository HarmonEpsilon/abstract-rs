use rocket::Response;
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

//Redirect to Home 
#[get("/")]
pub fn take_me_home() -> Redirect {
    Redirect::to("/home")
}

//Get request for Home, output Home template
#[get("/home")]
pub fn home() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    Template::render("main/home", &context)
}

//Get request for About page, output About template
#[get("/about")]
pub fn about() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    Template::render("docs/about", &context)
}

//Get request for Register page, output Register template
#[get("/register")]
pub fn sign_up() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    Template::render("user/register", &context)
}

//Get request for Sign In page, output Sign In Template
#[get("/signin")]
pub fn sign_in() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "[A] ABSTRACT");
    Template::render("user/signin", &context)
}
//Hook up files such as CSS and JavaScript
#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/styles/").join(file)).ok()
}