#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewRecord, Record};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn save_record_into_db(conn: &PgConnection, new_record: NewRecord) -> Record {
    use schema::teleinfo;

    diesel::insert_into(teleinfo::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving record")
}
