use std::str;
#[macro_use] extern crate diesel;
#[macro_use] extern crate hex_literal;
extern crate aes_soft as aes;
extern crate block_modes;

pub mod database;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
