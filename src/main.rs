#![feature(plugin)]
#![plugin(rocket_codegen)]

//Rocket crates
extern crate rocket_contrib;
extern crate rocket;

//Serde crates
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

//Diesel crates
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

//Custom modules
mod site_routes;
mod errors;
mod data;

use site_routes::routes::*;
use errors::error_codes::*;

//Gets all routes from modules
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![
        take_me_home,
        home,
        about,
        sign_up,
        files
    ]).catch(errors![
        not_found
    ])
}

//Launches server
fn main() {
    rocket().launch();
}
