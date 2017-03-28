use std::thread;
use std::env;

use r2d2;
use r2d2_diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;

//Pool struct, for creating new pools
pub struct Pool {
    pool: r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool { pool: establish_pool() }
    }
}

//Connect to database
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    MysqlConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}

//Database pool creation
pub fn establish_pool() -> r2d2::Pool<r2d2_diesel::ConnectionManager<MysqlConnection>>{ 
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let config = r2d2::Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    
    r2d2::Pool::new(config, manager).expect("Failed to create pool")
}