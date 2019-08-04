mod endpoint;
mod blockchain;
mod payload;

extern crate json;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate time;
extern crate sawtooth_sdk;
extern crate crypto;
extern crate futures;

use crate::futures::Stream;
use json::JsonValue;

use futures::{future, Future};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    app_name: String,
}

fn create_agent(item: web::Json<endpoint::AgentRequest>, data: web::Data<AppState>) -> HttpResponse {
    let agent: endpoint::AgentRequest = item.0;
    blockchain::run(agent.username.clone(), agent.password.clone());
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
    run()
}
