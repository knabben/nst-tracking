use diesel::prelude::*;


#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate time;
extern crate sawtooth_sdk;
extern crate crypto;
extern crate futures;

extern crate json;

mod endpoint;
mod blockchain;
mod payload;

use crate::futures::Stream;
use json::JsonValue;
use sawtooth_sdk::signing;
use futures::{future, Future};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use restapi;

struct AppState {
    app_name: String,
}

fn create_agent(item: web::Json<endpoint::CreateAgentRequest>, data: web::Data<AppState>) -> HttpResponse {
    let agent: endpoint::CreateAgentRequest = item.0;

    let transaction = blockchain::BCTransaction::new(
        "simple_supply".to_string(), "0.1".to_string(), "00".to_string());

    // Generate key pairs
    let (_private_key, _public_key) = transaction.generate_key_pair(&*transaction.context);

    // Transaction signer
    let crypto_factory = sawtooth_sdk::signing::CryptoFactory::new(&*transaction.context);
    let signer = crypto_factory.new_signer(&*_private_key);

    transaction.store(
        signer,
        _public_key.as_hex().to_string(),
        agent.username.clone()
    );

    let connection = restapi::establish_connection();
    restapi::create_auth(
        &*_public_key,
        &*_private_key,
        agent.password.to_string(),
        &connection
    );


    HttpResponse::Ok().json(agent) // <- send json response
}

fn run() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("haaaa"),
            })
            .service(
                web::scope("/api")
                    .route("/agent", web::post().to(create_agent))
            )
    }).bind("127.0.0.1:8086")
        .unwrap()
        .run()
        .unwrap();
}

fn main() {
    // treat variables and clap_app
    run()
}
