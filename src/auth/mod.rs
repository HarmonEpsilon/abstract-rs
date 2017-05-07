pub mod user;

use std::string::String;

use rocket::Request;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, Cookies};

use diesel::prelude::*;

use database::*;
use database::models::session_models::*;
use self::user::*;

//Finds a session then returns true if session is found
pub fn is_logged_in(mut cookies: &Cookies) -> Option<u64> {
    use database::models::session_models::sessions::table;
    let connection = connection::establish_connection();

    let session = cookies.find("session_token");

    let session_id = match session {
        Some(x) => session.unwrap().value(),
        None    => None,
    };

    let found_session = table.select(sessions::id.eq(session_id)).execute(&connection);

    match found_session {
        
    }
}

//Register a new user
#[post("/register", data="<new_user>")]
pub fn register_user(mut cook: &Cookies, new_user: Form<Register>) -> Redirect {
    let connection = connection::establish_connection();

    let user = create_user(
        &connection, 
        &new_user.get().get_username(), 
        &new_user.get().get_password(), 
        &new_user.get().get_email(),
    );
    
    let user_id = user.id.clone();
    let session = create_session(&connection, user_id);

    cook.add(Cookie::new("session_token", session.id.clone()));
    return Redirect::to("/home");
}

//Signs in a user
#[post("/signin", data="<user>")]
pub fn login(user: Form<Login>) -> Redirect {
    return Redirect::to("/home");
}