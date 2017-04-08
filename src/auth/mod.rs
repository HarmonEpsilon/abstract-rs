pub mod signin;
pub mod user;

use rocket::request::Form;
use rocket::response::Redirect;
//TODO: Implement session support once it lands in Rocket 0.3.0

use database::*;
use self::signin::*;
use self::user::*;

//Register a new user
#[post("/register", data="<new_user>")]
pub fn register_user(new_user: Form<Register>) -> Redirect {
    let connection = connection::establish_connection();

    //TODO: get successful user creation then test Session once that's working
    let user = create_user(
        &connection, 
        &new_user.get().get_username(), 
        &new_user.get().get_password(), 
        &new_user.get().get_email(),
    );
    
    return Redirect::to("/home");
}