use r2d2;
use r2d2_cypher::CypherConnectionManager;

use rocket::{http, request, Outcome, State};
use std::ops::Deref;

pub struct DbConnection(pub r2d2::PooledConnection<CypherConnectionManager>);

impl Deref for DbConnection {
    type Target = r2d2::PooledConnection<CypherConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> request::FromRequest<'a, 'r> for DbConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<DbConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}

type Pool = r2d2::Pool<CypherConnectionManager>;

pub fn init_pool(url: String) -> Pool {
    let manager = CypherConnectionManager { url: url };
    r2d2::Pool::new(manager).expect("db pool")
}
