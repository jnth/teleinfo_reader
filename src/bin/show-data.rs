extern crate diesel;
extern crate teleinfo_reader;

use self::diesel::prelude::*;
use self::teleinfo_reader::*;
use teleinfo_reader::settings::Settings;
use teleinfo_reader::models::Record;

fn main() {
    use teleinfo_reader::schema::teleinfo::dsl::*;

    let settings = Settings::read(None);
    let connection = establish_connection(&settings);
    let results = teleinfo
        .order_by(dt_utc.desc())
        .limit(1)
        .load::<Record>(&connection)
        .expect("Error loading data");

    if results.len() > 0 {
        println!("Last values saved in database:");
        for record in results {
            println!("{}", record);
        }
    } else {
        println!("There's no data in database.")
    }
}
