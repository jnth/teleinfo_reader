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
    let matches = App::new("Teleinfo Reader (show last record)")
        .version(crate_version!())
        .author("Jonathan Virga <jonathan.virga@gmail.com>")
        .about("Show the last values from the database")
        .arg(
            Arg::with_name("conf")
                .help("Force use of specific configuration file")
                .required(false)
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("file"),
        )
        .get_matches();

    let conf = matches.value_of("conf");
    let settings = Settings::read(conf);

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
