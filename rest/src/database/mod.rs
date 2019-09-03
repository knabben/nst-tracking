pub mod models;
pub mod schema;

use models::{NewAuth, NewProduct, NewBid};
use crate::database::models::{Auth, Product, Bid};
use schema::{auth, product, bid};

use std::error::Error;
use std::ops::Deref;

extern crate aes_soft as aes;
extern crate block_modes;

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use crate::diesel::RunQueryDsl;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use sawtooth_sdk::signing::{PrivateKey, PublicKey};

use diesel::{
    connection::Connection as _,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::run_pending_migrations;

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

pub fn run_all_migrations(database_url: &str) -> Result<(), Box<Error>> {
    let connection = PgConnection::establish(database_url)?;
    run_pending_migrations(&connection)?;
    Ok(())
}

pub fn create_connection_pool(database_url: &str) -> Result<ConnectionPool, r2d2::Error> {
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    Ok(ConnectionPool {
        pool: Pool::builder()
            .build(connection_manager)
            .map_err(|err| err)?,
    })
}

pub fn create_auth(
    public_key: &dyn PublicKey,
    private_key: &dyn PrivateKey,
    username: String,
    password: String,
    conn: &PgConnection,
) -> Auth {
    // Encrypt private key
    let key = hex!("ffffffffffffffffffffffffffffffff");
    let plaintext = private_key.as_slice();
    let iv = &public_key.as_hex().clone()[0..16];

    let cipher = Aes128Cbc::new_var(&key, iv.as_bytes()).unwrap();
    let mut buffer = [0u8; 64];

    let pos = plaintext.len();
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();

    let mut hasher = Sha512::new();
    hasher.input_str(&password);

    let auth_data = NewAuth {
        public_key: &public_key.as_hex(),
        username: &username,
        hashed_password: &hasher.result_str(),
        encrypted_private_key: &hex::encode(ciphertext),
    };

    diesel::insert_into(auth::table)
        .values(&auth_data)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_product(
    auth_id: i64,
    record_id: &str,
    title: &str,
    price: i64,
    latitude: i64,
    longitude: i64,
    conn: &PgConnection,
) -> Product {
    let product_data = NewProduct {
        record_id: record_id,
        auth_id: auth_id,
        title: title,
        price: price,
        latitude: latitude,
        longitude: longitude
    };

    diesel::insert_into(product::table)
        .values(&product_data)
        .get_result(conn)
        .expect("Error saving new product")
}

pub fn create_bid(
    auth_id: i64,
    product_id: i64,
    price: i64,
    conn: &PgConnection
) -> Bid {
    let bid_data = NewBid {
        product_id: product_id,
        price: price
    };
    diesel::insert_into(bid::table)
        .values(&bid_data)
        .get_result(conn)
        .expect("Error saving bid")
}

pub fn fetch_bid(
    _auth_id: i64,
    conn: &PgConnection,
) -> Vec<Bid> {
    use crate::diesel::ExpressionMethods;
    use self::schema::product::dsl::*;
    use crate::diesel::QueryDsl;
    use crate::diesel::query_dsl::BelongingToDsl;

    let p = product
        .filter(auth_id.ne_all(vec![_auth_id]))
        .first::<Product>(conn)
        .expect("Error product");

    let bids = Bid::belonging_to(&p)
        .load::<Bid>(conn)
        .expect("Error loading photos");

    bids
}

pub fn fetch_auth_resource(un: String, conn: &PgConnection) -> self::models::Auth {
    use self::schema::auth::dsl::*;
    use diesel::prelude::*;

    let results = auth
        .filter(username.eq(un))
        .first::<Auth>(conn)
        .expect("Error loading users.");

    results
}

pub fn fetch_products(_id: i64, conn: &PgConnection) -> Vec<Product> {
    use self::schema::product::dsl::*;
    use diesel::prelude::*;

    let results = product
        .filter(auth_id.ne_all(vec![_id]))
        .load::<Product>(conn)
        .expect("Error loading products.");

    results
}