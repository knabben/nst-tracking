use crate::database::schema::auth;
use serde::{Serialize};

#[derive(Debug, Queryable)]
pub struct Auth {
    pub id: i64,
    pub public_key: String,
    pub username: String,
    pub hashed_password: String,
    pub encrypted_private_key: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Product {
    pub id: i64,
    pub record_id: String,
    pub auth_id: i64,
    pub title: String,
    pub price: i64,
    pub latitude: i64,
    pub longitude: i64,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth<'a> {
    pub public_key: &'a str,
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub encrypted_private_key: &'a str,
}
