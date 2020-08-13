use regex::{Captures, Regex};
use std::env;

pub struct Settings {
    pub database_url: String,
    pub serial_path: String,
}

impl Settings {
    pub fn read(force_filename: Option<&str>) -> Settings {
        match force_filename {
            Some(filename) => {
                println!("Force configuration file: {}", &filename);
                dotenv::from_filename(filename).ok();
            }
            None => {
                println!("Use default configuration files");
                dotenv::from_filename("/etc/read-teleinfo.conf").ok();
                dotenv::from_filename(".env").ok();
            }
        };
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let serial_path = env::var("TELEINFO_SERIAL").expect("TELEINFO_SERIAL must be set");

        // Regex to parse database url in order to hide password
        let pattern = Regex::new(r"postgres://(?P<user>[^:@]+)(:(?P<password>[^:@]+))?@(?P<host>[^:@/]+)(:(?P<port>\d+))?/(?P<dbname>[^:@/]+)").unwrap();
        let masked = pattern.replace(&database_url, |caps: &Captures| match caps.name("port") {
            Some(_port) => format!(
                "postgres://{}@{}:{}/{}",
                &caps["user"], &caps["host"], &caps["port"], &caps["dbname"]
            ),
            None => format!(
                "postgres://{}@{}/{}",
                &caps["user"], &caps["host"], &caps["dbname"]
            ),
        });

        println!(" -> database_url: {}", &masked);
        println!(" -> serial_path: {}", &serial_path);
        Settings {
            database_url,
            serial_path,
        }
    }
}
