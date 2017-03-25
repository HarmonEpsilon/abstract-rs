use database::models::*;
use database::*;
use rocket::request::Form;
use rocket::response::Redirect;
use std::io::{stdin, Read};

//Register a new user
#[post("/register", data="<new_user>")]
pub fn register_user<'a>(new_user: Form<'a, NewUser<'a>>) -> Redirect {
    let connection = establish_connection();
    let user = new_user.get();

    Redirect::to("/home")
}
