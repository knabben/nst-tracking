use jwt::{
    Header,
    Registered,
    Token,
};
extern crate aes_soft as aes;
extern crate block_modes;

use diesel::{pg::PgConnection};
use crypto::sha2::Sha256;
use crate::routes::{Response};
use crate::database::{fetch_auth_resource};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};



use aes::Aes128;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn generate_jwt(username: String, jwt_token: &[u8]) -> Response {
    let header: Header = Default::default();
    let claims = Registered {
        iss: Some(username),
        ..Default::default()
    };
    let token = Token::new(header, claims);
    Response{ token: token.signed(&jwt_token, Sha256::new()).unwrap() }
}

pub fn deserialize_jwt(secret: &[u8], jwt_token: &str) -> (bool, String) {
    let jwt_slice = &jwt_token[4..];
    let token = Token::<Header, Registered>::parse(jwt_slice).unwrap();

    if (token.verify(&secret, Sha256::new())) {
        (true, token.claims.iss.unwrap())
    } else {
        (false, "".to_string())
    }
}

pub fn decode_private_key(username: String, db: &diesel::PgConnection) -> std::vec::Vec<u8> {
    let auth_info = fetch_auth_resource(username.clone(), db);

    let key = hex!("ffffffffffffffffffffffffffffffff");
    let iv = &auth_info.public_key.clone()[0..16];

    let cipher = Aes128Cbc::new_var(&key, iv.as_bytes()).unwrap();

    let mut buffer = [0u8; 48];
    let decoded_key = hex::decode(&auth_info.encrypted_private_key.as_bytes()).unwrap();

    buffer.copy_from_slice(&decoded_key);
    let decrypt_buffer = &mut buffer.clone();

    let private_key = cipher.decrypt(decrypt_buffer).unwrap();
    private_key.to_owned()
}