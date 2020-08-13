#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewRecord, Record};
use self::settings::Settings;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod models;
pub mod schema;
pub mod settings;

pub fn establish_connection(settings: &Settings) -> PgConnection {
    PgConnection::establish(&settings.database_url)
        .expect(&format!("Error connecting to {}", settings.database_url))
}

pub fn save_record_into_db(conn: &PgConnection, new_record: NewRecord) -> Record {
    use schema::teleinfo;

    diesel::insert_into(teleinfo::table)
        .values(&new_record)
        .get_result(conn)
        .expect("Error saving record")
}
