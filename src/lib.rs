#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{Reading,NewReading};

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_reading(conn: &PgConnection, reading: NewReading) -> Reading {
  use schema::readings;
  diesel::insert_into(readings::table)
    .values(&reading)
    .get_result(conn)
    .expect("Error saving new reading")
}

