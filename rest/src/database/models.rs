
use crate::database::schema::{auth};

#[derive(Queryable)]
pub struct Auth {
    pub public_key: String,
    pub username: String,
    pub hashed_password: String,
    pub encrypted_private_key: String,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth<'a> {
    pub public_key: &'a str,
    pub username: &'a str,
    pub hashed_password: &'a str,
    pub encrypted_private_key: &'a str,
}