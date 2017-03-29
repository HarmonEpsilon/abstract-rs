use std::io::{stdin, Read};
use std::string::String;

use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::response::{Redirect, Flash};
//TODO: figure out what the hell is happening with Session
use rocket::http::Cookies;
use rocket::Outcome;

use database::*;

#[derive(FromForm)]
pub struct Register {
    pub user: String,
    pub pass: String,
    pub email: String,
}

//Register a new user
#[post("/register", data="<new_user>")]
pub fn register_user(new_user: Form<Register>) -> Redirect {
    let connection = connection::establish_connection();
    let user = new_user.get();

    //TODO: get successful user creation then test Session once that's working
    let new_hombre = create_user(&connection, &user.user, &user.pass, &user.email);
    
    Redirect::to("/home")
}
