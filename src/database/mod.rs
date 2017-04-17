pub mod schema;
pub mod models;
pub mod connection;

//Standard imports
use rand::os::OsRng;
use rand::Rng;
use rand::Rand;
use std::cell::RefCell;

//Diesel imports
use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

//Custom imports
use self::models::user_models::{User, NewUser};
use self::models::session_models::{Session, NewSession};

//Other imports
use base64;
use bcrypt::{DEFAULT_COST, hash};

//Secure generator
fn secure_gen<T: Rand>() -> T {
    thread_local! {
        static GENERATOR: RefCell<OsRng> = RefCell::new(OsRng::new().unwrap());
    }

    return GENERATOR.with(|g| g.borrow_mut().gen());
}

//Create a new user and insert it into the Users table
pub fn create_user(conn: &MysqlConnection, user: &str, pass: &str, email: &str) -> User {
    use database::schema::users::dsl::{users, id};

    let pw = match hash(pass, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!()
    };

    let new_user = NewUser {
        user: user,
        pass: pw.as_str(),
        email: email
    };

    diesel::insert(&new_user).into(users)
        .execute(conn)
        .expect("Error creating new user");

    return users.order(id.desc()).first(conn).unwrap();
}

//Creates a new session using a user id and generates a session ID using OsRng
pub fn create_session(conn: &MysqlConnection, user_id: i32) -> Session {
    use database::models::session_models::sessions::table;

    let identity: [u8; 24] = secure_gen();
    let new_session = NewSession {
        id: base64::encode_config(&identity, base64::URL_SAFE),
        user_id: user_id,
    };

    diesel::insert(&new_session).into(table)
            .execute(conn)
            .expect("Error creating new session");

    return table.first(conn).unwrap();
}