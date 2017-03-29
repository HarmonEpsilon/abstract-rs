use std::env;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;

//Connect to database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    MysqlConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}