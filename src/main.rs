/**
 * @brief Main file for execution of Abstract webserver, utilizing Nickel
 * @description
 * 
 * Creates a webserver using Nickel.rs that directs to HTML stored in assets folder
 *
 * @author Landon Mote
 * @date 3.13.2017
 */
#[macro_use]
extern crate nickel;
extern crate mysql;
extern crate hyper;
extern crate rustc_serialize;
#[macro_use(bson, doc)]
extern crate bson;

/* Imports */
mod server;
use server::serv::*;
use std::{env, str};

/* Main function */
fn main() {
    let port = env::var("PORT").map(|s| s.parse().unwrap()).unwrap_or(3000);
    let address = &format!("127.0.0.1:{}", port);

    serv::start_server(address).unwrap();
}