#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

//Custom modules
mod home;
mod auth;
mod errors;

use home::home_routes::*;
use errors::error_codes::*;

//Gets all routes from modules
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        take_me_home,
        home,
        about,
        files
    ]).catch(errors![
        not_found
    ])
}

//Launches server
fn main() {
    println!("Starting server...");
    rocket().launch();
}
