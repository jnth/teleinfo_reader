extern crate diesel;
extern crate teleinfo_reader;

use self::teleinfo_reader::*;
// use self::models;
use self::diesel::prelude::*;
use teleinfo_reader::models::Record;

fn main() {
    use teleinfo_reader::schema::teleinfo::dsl::*;

    let connection = establish_connection();
    let results = teleinfo.load::<Record>(&connection)
        .expect("Error loading data");

    let plr = match results.len() {
        0 | 1 => "",
        _ => "s"
    };
    println!("=> {} row{}", results.len(), plr);
    for record in results {
        println!("{:?}", record);
    }
}