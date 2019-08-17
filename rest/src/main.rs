#[macro_use]
extern crate clap;

#[macro_use] extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use crypto::sha2::Sha256;
use jwt::{
    Header,
    Registered,
    Token,
};

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate time;
extern crate sawtooth_sdk;
extern crate crypto;
extern crate futures;

extern crate json;
mod validator;
mod config;
mod endpoint;
mod blockchain;
mod payload;
mod database;

use std::error::Error;

use log::Level;
use simple_logger;
use clap::{App as ClapApp, Arg};

use crate::validator::SawtoothConnection;
use crate::database::run_all_migrations;

use crate::futures::Stream;
use sawtooth_sdk::signing::{CryptoFactory};
use futures::{future, Future};

use actix::{Addr, SyncArbiter};
use actix_web::{http, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;

use crate::database::ConnectionPool;

use actix::{Actor, Context, SyncContext};
use sawtooth_sdk::messaging::stream::MessageSender;

use restapi;

pub struct AppState {
    //sawtooth_connection: Box<dyn MessageSender + Send>,
    database_connection: database::ConnectionPool,
}

pub struct DbExecutor {
    connection_pool: ConnectionPool,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(connection_pool: ConnectionPool) -> DbExecutor {
        DbExecutor { connection_pool }
    }
}

pub struct SawtoothMessageSender {
    sender: Box<dyn MessageSender>,
}

impl Actor for SawtoothMessageSender {
    type Context = Context<Self>;
}

impl SawtoothMessageSender {
    pub fn new(sender: Box<dyn MessageSender>) -> SawtoothMessageSender {
        SawtoothMessageSender { sender }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    token: String,
}

fn create_agent(
    item: web::Json<endpoint::CreateAgentRequest>, 
    data: web::Data<AppState>
) -> HttpResponse {
    let agent: endpoint::CreateAgentRequest = item.0;

    let transaction = blockchain::BCTransaction::new(
        "trade".to_string(), 
        "1.0".to_string(),
        "00".to_string()
        );
    
    // Generate key pairs
    let (_private_key, _public_key) = transaction.generate_key_pair(&*transaction.context);

    // Transaction signer
    let crypto_factory = CryptoFactory::new(&*transaction.context);
    let signer = crypto_factory.new_signer(&*_private_key);

    transaction.store(
        signer,
        _public_key.as_hex().to_string(),
        agent.username.clone()
    );

    let connection = &data.database_connection;
    database::create_auth(
        &*_public_key,
        &*_private_key,
        agent.password.to_string(),
        &connection.get().unwrap()
    );

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

fn run() -> Result<(), Box<Error>> {
    let matches = ClapApp::new("app")
        .arg(Arg::with_name("connect")
            .help("Endpoint validator")
            .short("C")
            .default_value("tcp://127.0.0.1:4004")
            .takes_value(true)
        )
        .arg(Arg::with_name("database_url")
            .help("Database URL endpoint")
            .default_value("postgres://postgres:postgres@localhost/restapi")
            .takes_value(true)
        )
        .arg(Arg::with_name("bind")
            .help("Bind REST endpoint")
            .short("b")
            .default_value("0.0.0.0:8086")
            .takes_value(true)
        )
        .arg(Arg::with_name("verbose")
            .help("Bind REST endpoint")
            .short("v")
            .multiple(true)
            .takes_value(true)
        ).get_matches();

    let log_level = match matches.occurrences_of("verbose") {
        0 => Level::Warn,
        1 => Level::Info,
        2 => Level::Debug,
        _ => Level::Trace,
    };

    simple_logger::init_with_level(log_level)?;

    let config = config::Config{
        rest_api_endpoint: matches.value_of("bind").unwrap().to_string(),
        validator_endpoint: matches.value_of("connect").unwrap().to_string(),
        database_endpoint: matches.value_of("database_url").unwrap().to_string(),
    };

    let database_url = config.database_endpoint();

    // Run diesel migrations
    run_all_migrations(&database_url);

    // Sawtooth connection
    let sawtooth_connection = SawtoothConnection::new(config.validator_endpoint());
    let connection_pool = database::create_connection_pool(&database_url)?;

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                //sawtooth_connection: sawtooth_connection.get_sender(),
                database_connection: connection_pool.clone(),
            })
            .wrap(
                Cors::new()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
            )
            .service(
                web::scope("/api")
                    .route("/agent", web::post().to(create_agent))
            )
    }).bind(config.rest_api_endpoint())
        .unwrap()
        .run()
        .unwrap();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        std::process::exit(1);
    }
}