pub mod models;
pub mod schema;

use models::NewAuth;
use schema::auth;

use std::error::Error;
use std::ops::Deref;

extern crate aes_soft as aes;
extern crate block_modes;

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

use crate::diesel::RunQueryDsl;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

use crate::database::models::{Auth};
use sawtooth_sdk::signing::{PrivateKey, PublicKey};

use diesel_migrations::run_pending_migrations;
use diesel::{
    connection::Connection as _,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub fn run_all_migrations(database_url: &str) -> Result<(), Box<Error>> {
  let connection = PgConnection::establish(database_url)?;
  run_pending_migrations(&connection)?;
  Ok(())
}

pub fn create_connection_pool(database_url: &str) -> Result<ConnectionPool, r2d2::Error> {
  let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

  Ok(ConnectionPool {
    pool: Pool::builder().build(connection_manager).map_err(|err| err)?,
  })
}

pub fn create_auth(
    public_key: &dyn PublicKey,
    private_key: &dyn PrivateKey,
    password: String,
    conn: &PgConnection
) -> Auth {
    // Encrypt private key
    //let key = hex!("ffffffffffffffffffffffffffffffff");
    // let plaintext = private_key.as_slice();
    // let iv = &public_key.as_hex().clone()[0..16];

    //let cipher = Aes128Cbc::new_var(&key, iv.as_bytes()).unwrap();

    // let mut buffer = [0u8; 64];

    // let pos = plaintext.len();
    // buffer[..pos].copy_from_slice(plaintext);
    // let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    // encrypt with AWS MODE CBC private_key
    // return bcrypt.hashpw(bytes(password, 'utf-8'), bcrypt.gensalt())

    let authx = NewAuth {
        public_key: &public_key.as_hex(),
        hashed_password: &"hashed".to_string(),
        encrypted_private_key: &"hased".to_string(), //&hex::encode(ciphertext),
    };

    diesel::insert_into(auth::table).values(&authx).get_result(conn).expect("Error saving new post")
}

pub struct Connection(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn get(&self) -> Result<Connection, r2d2::Error> {
        self.pool.get().map(Connection).map_err(|err| err)
    }
}