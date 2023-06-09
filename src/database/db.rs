
// pub mod models;
// pub mod schema;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::env;

pub fn connect() -> MysqlConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}