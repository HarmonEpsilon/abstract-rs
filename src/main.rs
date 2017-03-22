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

use home::home_routes::*;

//Gets all routes from modules
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        take_me_home,
        home,
        about,
        files
    ])
}

//Launches server
fn main() {
    rocket().launch();
}
