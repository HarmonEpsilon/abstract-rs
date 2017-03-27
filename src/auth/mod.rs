use database::models::*;
use database::*;
use rocket::request::Form;
use rocket::response::Redirect;
use std::io::{stdin, Read};

#[derive(FromForm)]
pub struct Register {
    pub user: String,
    pub pass: String,
    pub email: String,
    pub agree: bool
}

//Register a new user
#[post("/register", data="<new_user>")]
pub fn register_user<'a>(new_user: Form<Register>) -> Redirect {
    let user = new_user.get();

    Redirect::to("/home")
}
