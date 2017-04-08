use rocket::Request;
use rocket_contrib::Template;

use std::collections::HashMap;

//404 Error page
#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut path = HashMap::new();
    path.insert("path", req.uri().as_str());
    path.insert("title", "[A] ABSTRACT");
    return Template::render("error/404", &path);
}

//422 Unprocessable Entity
#[error(422)]
pub fn unprocessable_entity() -> Template {
    let mut path = HashMap::new();
    path.insert("title", "[A] ABSTRACT");
    return Template::render("error/422", &path);
}