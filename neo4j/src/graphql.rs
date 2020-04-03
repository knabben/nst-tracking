use crate::db;
use crate::schema;
use rocket::{response::content, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

use rocket::http::Method;
use rocket::response::Responder;

pub fn cors_options() -> CorsOptions {
    let allowed_origins = AllowedOrigins::All;

    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
}

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler<'r>(
    connection: db::DbConnection,
    request: juniper_rocket::GraphQLRequest,
    schema: State<schema::Schema>,
) -> impl Responder<'r> {
    let response = request.execute(&schema, &connection);
    let options = cors_options().to_cors().unwrap();
    options.respond_owned(|guard| guard.responder(response))
}

#[options("/graphql")]
pub fn options<'r>() -> impl Responder<'r> {
    let options = cors_options().to_cors()?;
    options.respond_owned(|guard| guard.responder(()))
}
