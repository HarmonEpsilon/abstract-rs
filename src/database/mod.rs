pub mod schema;
pub mod models;

//Diesel imports
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

//Custom imports
use self::models::{User, NewUser};

//Establish connection to database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

//Create a new user and insert it into the Users table
pub fn create_user(conn: &MysqlConnection, user: &str, pass: &str, email: &str) -> User {
    use database::schema::users::dsl::{users, id};

    let new_user = NewUser {
        user: user,
        pass: pass,
        email: email
    };

    diesel::insert(&new_user).into(users)
        .execute(conn)
        .expect("Error creating new user");

    users.order(id.desc()).first(conn).unwrap()
}