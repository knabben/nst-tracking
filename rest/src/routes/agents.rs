use crate::{AppState};
use crate::routes::{CreateAgentRequest, AuthorizeAgentRequest};
use crate::database::{create_auth, fetch_auth_resource};
use crate::blockchain::transaction::{BCTransaction};

use crate::routes::utils::{generate_jwt};
use sawtooth_sdk::signing::{CryptoFactory};
use actix_web::{http, web, HttpResponse};

use crypto::digest::Digest;
use crypto::sha2::{Sha256, Sha512};

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
    let username = agent.username.clone();
    
    // Save transaction on sawtooth
    transaction.store(
        signer,
        _public_key.as_hex().to_string(),
        &username,
        data.sawtooth_connection.clone()
    );

    // Save on database
    let connection = &data.database_connection;
    create_auth(
        &*_public_key,
        &*_private_key,
        username.clone(),
        agent.password.to_string(),
        &connection.get().unwrap()
    );

    HttpResponse::Ok().json(generate_jwt(username.clone(), &data.jwt_sign.as_bytes()))
}

pub fn authentication(
    item: web::Json<AuthorizeAgentRequest>, data: web::Data<AppState>
) -> HttpResponse {
    let mut hasher = Sha512::new();
    let agent: AuthorizeAgentRequest = item.0;
    let auth_info = fetch_auth_resource(agent.username.clone(), &data.database_connection.get().unwrap());

    hasher.input_str(&agent.password);
    if hasher.result_str() != auth_info.hashed_password {
        HttpResponse::NotFound().json("Not found")
    } else {
        let token = generate_jwt(agent.username.clone(),  &data.jwt_sign.as_bytes());
        HttpResponse::Ok().json(token)
    }
}