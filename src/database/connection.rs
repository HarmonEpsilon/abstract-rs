use std::thread;
use std::env;

use r2d2;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;

pub fn database_connection() { 
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let config = r2d2::Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();
            assert!(connection.is_ok());
        });
    }
}