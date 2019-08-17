pub mod models;
pub mod schema;

use std::error::Error;
use std::ops::Deref;

use diesel_migrations::run_pending_migrations;
use diesel::{
    connection::Connection as _,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};

pub fn run_all_migrations(database_url: &str) -> Result<(), Box<Error>> {
  let connection = PgConnection::establish(database_url)?;
  run_pending_migrations(&connection)?;
  Ok(())
}

pub fn create_connection_pool(database_url: &str) -> Result<ConnectionPool, r2d2::Error> {
  let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

  Ok(ConnectionPool {
    pool: Pool::builder().build(connection_manager).map_err(|err| err)?,

  })
}

pub struct Connection(PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ConnectionPool {
    pub fn get(&self) -> Result<Connection, r2d2::Error> {
        self.pool.get().map(Connection).map_err(|err| err)
    }
}