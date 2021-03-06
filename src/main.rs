#![feature(plugin, custom_derive, custom_attribute)]
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

//Other crates
extern crate rand;
extern crate base64;
extern crate bcrypt;
extern crate cookie;

//Custom modules
mod routes;
mod errors;
mod database;
mod auth;
mod misc;

//Custom uses
use routes::*;
use errors::*;
use auth::*;

//Gets all routes from modules
fn rocket() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![
        take_me_home,
        home,
        about,
        sign_up,
        sign_in,
        register_user,
        login,
        omnibus,
        group,
        files
    ]).catch(errors![
        not_found,
        unprocessable_entity
    ]);
}

//Launches server
fn main() {
    rocket().launch();
}
