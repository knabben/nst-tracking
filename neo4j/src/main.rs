#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)]
mod tests;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;

use rocket::fairing::AdHoc;
use rocket::http::{HeaderMap, Method, Status};

extern crate clap;
extern crate juniper_rocket;
use clap::{App, Arg};
mod db;
mod graphql;
mod schema;

fn main() {
    let matches = App::new("Thefind")
        .version("1.0")
        .author("Amim Knabben <amim.knabben@gmail.com>")
        .about("Find your resource")
        .arg(
            Arg::with_name("neo4j")
                .short("n")
                .long("neo4j")
                .value_name("FILE")
                .help("Set the Database URL")
                .takes_value(true),
        )
        .get_matches();

    let neo4j = matches.value_of("neo4j").unwrap_or("http://").to_string();

    rocket::ignite()
        .manage(db::init_pool(neo4j))
        .manage(schema::Schema::new(schema::Query, schema::Mutation))
        .mount(
            "/",
            rocket::routes![
                graphql::graphiql,
                graphql::post_graphql_handler,
                graphql::options
            ],
        )
        .manage(graphql::cors_options().to_cors().expect("To not fail"))
        .attach(AdHoc::on_response("Authentication", |req, res| {
            if req.method() == Method::Post && req.uri().path() == "/graphql" {
                let headers = req.headers();
                match headers.get_one("Referer") {
                    Some(val) => {
                        if !val.contains("/human/") {
                            let status = check_headers(headers);
                            res.set_status(status)
                        }
                    }
                    None => println!("no referer"),
                }
            }
        }))
        .launch();
}

fn check_headers<'g>(headers: &HeaderMap) -> Status {
    match headers.get_one("Authorization") {
        Some(val) => {
            if val.contains("mytoken") {
                Status::Forbidden
            } else {
                Status::Ok
            }
        }
        None => Status::Forbidden,
    }
}
