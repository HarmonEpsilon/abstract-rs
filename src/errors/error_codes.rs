use rocket::Request;
use rocket_contrib::Template;

use std::collections::HashMap;

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut path = HashMap::new();
    path.insert("path", req.uri().as_str());
    Template::render("error/404", &path)
}