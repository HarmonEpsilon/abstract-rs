use std::thread;

use r2d2;
use diesel::mysql::MysqlConnection;
use r2d2_diesel::ConnectionManager;

pub fn database_connection() { 
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new("mysql://localhost:3306/");
    let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool");

    for _ in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let connection = pool.get();
            assert!(connection.is_ok());
        });
    }
}