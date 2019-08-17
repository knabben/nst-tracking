#[macro_use] extern crate clap;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate time;
extern crate sawtooth_sdk;
extern crate crypto;
extern crate futures;

extern crate json;
mod validator;
mod config;
mod blockchain;
mod payload;
mod database;
mod routes;

use std::error::Error;

use log::Level;
use simple_logger;
use clap::{App as ClapApp, Arg};

use crate::validator::SawtoothConnection;
use crate::database::run_all_migrations;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use routes::agents::{create_agent};

pub struct AppState {
    sawtooth_connection: SawtoothConnection,
    database_connection: database::ConnectionPool,
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
    match run_all_migrations(&database_url) {
        Ok(m) => m,
        Err(e) => error!("Error {:?}", e),
    }

    // Sawtooth connection
    let connection_pool = database::create_connection_pool(&database_url)?;
    let sawtooth_connection = SawtoothConnection::new(config.validator_endpoint());

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                sawtooth_connection: sawtooth_connection.clone(),
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