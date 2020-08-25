extern crate diesel;
extern crate teleinfo_reader;

use self::diesel::prelude::*;
use self::teleinfo_reader::*;
use clap::{crate_version, App, Arg};
use teleinfo_reader::models::Record;
use teleinfo_reader::settings::Settings;

fn main() {
    use teleinfo_reader::schema::teleinfo::dsl::*;

    // Arguments and options
    let matches = App::new("Teleinfo Reader (dump records)")
        .version(crate_version!())
        .author("Jonathan Virga <jonathan.virga@gmail.com>")
        .about("Dump all data")
        .arg(
            Arg::with_name("conf")
                .help("Force use of specific configuration file")
                .required(false)
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("file"),
        )
        .arg(
            Arg::with_name("output")
                .help("Name of output file to create")
                .required(true)
                .index(1),
        )
        .get_matches();

    let output = matches.value_of("output").expect("Missing output path");
    let conf = matches.value_of("conf");
    let settings = Settings::read(conf);

    let connection = establish_connection(&settings);
    let results = teleinfo
        .load::<Record>(&connection)
        .expect("Error loading data");

    if results.len() > 0 {
        let mut wtr = csv::Writer::from_path(output).expect("Cannot write CSV data");
        for record in results {
            wtr.serialize(record).expect("Error when writing record");
        }
        wtr.flush().unwrap();
        println!("Dump saved to {}", output);
    } else {
        println!("There's no data in database.");
    }
}
