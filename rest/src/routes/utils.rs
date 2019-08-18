use jwt::{
    Header,
    Registered,
    Token,
};
use crypto::sha2::Sha256;
use crate::routes::{Response};

pub fn generate_jwt(username: String, jwt_token: &[u8]) -> Response {
    let header: Header = Default::default();
    let claims = Registered {
        iss: Some(username),
        ..Default::default()
    };
    let token = Token::new(header, claims);
    Response{ token: token.signed(&jwt_token, Sha256::new()).unwrap() }
}