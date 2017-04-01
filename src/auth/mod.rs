use std::string::String;

use rocket::request::Form;
use rocket::response::Redirect;
//TODO: figure out what the hell is happening with Session
use rocket::http::Cookies;

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

    //TODO: get successful user creation then test Session once that's working
    let user = create_user(
        &connection, 
        &new_user.get().user, 
        &new_user.get().pass, 
        &new_user.get().email,
    );
    
    Redirect::to("/home")
}

//User State
pub fn is_logged_in(is_true: i32) -> bool {
    //TODO: once Session is working, check if active session, THEN return true
    //This is currently only for testing purposes
    if is_true == 0 {
        false
    } else if is_true == 1 {
        true
    } else {
        false
    }
} 