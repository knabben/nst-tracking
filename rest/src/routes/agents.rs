use crate::routes::{CreateAgentRequest, Response};
use crate::{AppState};
use crate::database::{create_auth};
use crate::blockchain::{BCTransaction};

use sawtooth_sdk::signing::{CryptoFactory};
use actix_web::{http, web, HttpResponse};

use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};


pub fn create_agent(
    item: web::Json<CreateAgentRequest>, data: web::Data<AppState>
) -> HttpResponse {
    let agent: CreateAgentRequest = item.0;

    let transaction = BCTransaction::new(
        "trade".to_string(), 
        "1.0".to_string(),
        "00".to_string()
        );
    
    // Generate key pairs
    let (_private_key, _public_key) = transaction.generate_key_pair(&*transaction.context);

    // Transaction signer
    let crypto_factory = CryptoFactory::new(&*transaction.context);
    let signer = crypto_factory.new_signer(&*_private_key);

    // Save transaction on sawtooth
    transaction.store(
        signer,
        _public_key.as_hex().to_string(),
        agent.username.clone(),
        data.sawtooth_connection.clone()
    );


    // Save on database
    let connection = &data.database_connection;
    create_auth(
        &*_public_key,
        &*_private_key,
        agent.password.to_string(),
        &connection.get().unwrap()
    );

    // Generate token
    let header: Header = Default::default();
    let claims = Registered {
        iss: Some(agent.username.clone()),
        sub: Some(agent.username.clone()),
        ..Default::default()
    };
    let token = Token::new(header, claims);
    let value = Response{
        token: token.signed(&*_private_key.as_slice(), Sha256::new()).unwrap()
    };
 
    HttpResponse::Ok().json(value)
}