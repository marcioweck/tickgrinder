// functions for communicating with the postgresql database

#![allow(unused_must_use)]

use postgres::{Connection, SslMode};
use postgres::error;

#[derive(Clone)]
pub struct PostgresConf {
    pub postgres_user: &'static str,
    pub postgres_password: &'static str,
    pub postgres_url: &'static str,
    pub postgres_port: i32,
    pub postgres_db: &'static str
}

pub fn get_client(pg_conf: PostgresConf) -> Result<Connection, error::ConnectError> {
    let conn_string = format!("postgres://{}:{}@{}:{}/{}",
        pg_conf.postgres_user,
        pg_conf.postgres_password,
        pg_conf.postgres_url,
        pg_conf.postgres_port,
        pg_conf.postgres_db
    );

    Connection::connect(conn_string.as_str(), SslMode::None)
}

/***************************
   TICK-RELATED FUNCTIONS
***************************/

// Creates a new table for ticks with given symbol
pub fn init_tick_table(symbol: &str, client: &Connection, pg_user: &'static str) {
    let query1 = format!(
    "CREATE TABLE IF NOT EXISTS ticks_{}
    (
      tick_time bigint NOT NULL PRIMARY KEY UNIQUE,
      bid double precision NOT NULL,
      ask double precision NOT NULL
    )
    WITH (
      OIDS=FALSE
    );", symbol);
    let query2 = format!(
    "ALTER TABLE ticks_{}
      OWNER TO {};", symbol, pg_user);
    client.execute(query1.as_str(), &[])
        .map_err(|_| println!("Error while querying postgres to set up tick table"));
    client.execute(query2.as_str(), &[])
        .map_err(|_| println!("Error while querying postgres to set up tick table"));
}

/***************************
  ADMINISTRATIVE FUNCTIONS
***************************/

// Drops all tables in the database, resetting it to defaults
pub fn reset_db(client: &Connection, pg_user: &'static str) -> Result<(), error::Error> {
    let query = format!("DROP SCHEMA public CASCADE;
        CREATE SCHEMA public AUTHORIZATION {};
        ALTER SCHEMA public OWNER TO {};
        GRANT ALL ON SCHEMA public TO {};",
            pg_user, pg_user, pg_user);
    client.batch_execute(query.as_str())
}
