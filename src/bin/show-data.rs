extern crate chrono_tz;
extern crate diesel;
extern crate teleinfo_reader;

use self::diesel::prelude::*;
use self::teleinfo_reader::*;
use chrono::prelude::*;
use chrono::Utc;
use chrono_tz::Europe::Paris;
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

    println!();
    if results.len() > 0 {
        println!("Last values saved in database:");
        for record in results {
            let khcjb = record.hcjb as f64 / 1000.;
            let khpjb = record.hpjb as f64 / 1000.;
            let khcjw = record.hcjw as f64 / 1000.;
            let khpjw = record.hpjw as f64 / 1000.;
            let khcjr = record.hcjr as f64 / 1000.;
            let khpjr = record.hpjr as f64 / 1000.;
            let utc = TimeZone::from_utc_datetime(&Utc, &record.dt_utc);
            let local = utc.with_timezone(&Paris);

            println!(
                " Date/time -> UTC   | {}",
                record.dt_utc.format("%Y-%m-%d %H:%M:%S")
            );
            println!(
                "           -> Paris | {}",
                local.format("%Y-%m-%d %H:%M:%S")
            );
            println!("               ADCO | {}", record.adco);
            println!("      Bleu HC index | {} kWh", khcjb);
            println!("      Bleu HP index | {} kWh", khpjb);
            println!("     Blanc HC index | {} kWh", khcjw);
            println!("     Blanc HP index | {} kWh", khpjw);
            println!("     Rouge HC index | {} kWh", khcjr);
            println!("     Rouge HP index | {} kWh", khpjr);
        }
    } else {
        println!("There's no data in database.")
    }
}
