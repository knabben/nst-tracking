#[macro_use]
extern crate diesel;
use std::str;

#[macro_use] extern crate hex_literal;
extern crate aes_soft as aes;
extern crate block_modes;
use sawtooth_sdk::signing::{PrivateKey, PublicKey};

pub mod schema;
pub mod models;

use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use aes::Aes128;


use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Auth};

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_auth(
    public_key: &PublicKey,
    private_key: &PrivateKey,
    password: String,
    conn: &PgConnection
) -> Auth {
    use schema::auth;

    // Encrypt private key
    let key = hex!("ffffffffffffffffffffffffffffffff");
    let plaintext = private_key.as_slice();
    let iv = &public_key.as_hex().clone()[0..16];

    let cipher = Aes128Cbc::new_var(&key, iv.as_bytes()).unwrap();

    let mut buffer = [0u8; 64];

    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    // encrypt with AWS MODE CBC private_key
    // return bcrypt.hashpw(bytes(password, 'utf-8'), bcrypt.gensalt())

    let authx = models::NewAuth {
        public_key: &public_key.as_hex(),
        hashed_password: &"hashed".to_string(),
        encrypted_private_key: &hex::encode(ciphertext),
    };

    diesel::insert_into(auth::table)
        .values(&authx)
        .get_result(conn)
        .expect("Error saving new post")
}
