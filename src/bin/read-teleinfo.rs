use chrono::{DateTime, Utc};
use clap::{App, Arg};
use cron::Schedule;
use log::debug;
use serialport::prelude::*;
use std::io;
use std::str::FromStr;
use std::time::Duration;
use teleinfo_reader::models::NewRecord;
use teleinfo_reader::settings::Settings;
use teleinfo_reader::{establish_connection, save_record_into_db};

struct Events {
    schedule: Schedule,
    dt_utc: DateTime<Utc>,
}

impl Events {
    /// Create events
    fn new(cron_expression: &str) -> Events {
        let schedule = Schedule::from_str(cron_expression).unwrap();
        let dt_utc = schedule.upcoming(Utc).next().unwrap();
        Events { schedule, dt_utc }
    }

    /// Update to the next scheduled event
    fn next(&mut self) {
        let dt_utc = self.schedule.upcoming(Utc).next().unwrap();
        self.dt_utc = dt_utc;
    }

    /// Check if the event is passed
    fn passed(&self) -> bool {
        self.dt_utc <= Utc::now()
    }
}

fn main() {
    // Arguments and options
    let matches = App::new("Teleinfo Reader -- read serial data")
        .version("0.3.0")
        .author("Jonathan Virga <jonathan.virga@gmail.com>")
        .about("Read teleinfomation data from serial device")
        .arg(
            Arg::with_name("device")
                .help("Path of serial device")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Verbose mode")
                .short("v")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("conf")
                .help("Force use of specific configuration file")
                .required(false)
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("file")
        )
        .get_matches();

    // Configuration
    let conf = matches.value_of("conf");
    let settings = Settings::read(conf);

    let device = match matches.value_of("device") {
        Some(path) => path,
        None => &settings.serial_path,
    };
    let verbose = matches.is_present("verbose");

    let baud_rate = 1200;
    let cron_expression = "0 * * * * * *"; // every minutes
    let mut events = Events::new(cron_expression);

    let conn = establish_connection(&settings);

    let settings = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Seven,
        flow_control: FlowControl::None,
        parity: Parity::Even,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };
    let port = serialport::open_with_settings(&device, &settings);
    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1];
            let mut serial_data: Vec<u8> = Vec::new();
            let mut started: bool = false;
            events.next();

            println!("Listening data on {} at baud {}", &device, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(_t) => {
                        let c = &serial_buf[0];
                        if c == &2 {
                            debug!("Get start character of a record");
                            serial_data.clear();
                            started = true;
                        } else if c == &3 && started {
                            debug!("Get end of record character");
                            match NewRecord::from_string(
                                String::from_utf8_lossy(&serial_data).into_owned(),
                            ) {
                                Some(new_record) => {
                                    let now = Utc::now().format("%Y-%m-%d %H:%M:%S");
                                    if events.passed() {
                                        let record = save_record_into_db(&conn, new_record);
                                        if verbose {
                                            println!(
                                                "Get new record at {} => saved into database: {}",
                                                now, record
                                            );
                                        }
                                        events.next();
                                    } else {
                                        if verbose {
                                            println!("Get new record at {} => skipped", now);
                                        }
                                    }
                                }
                                None => {
                                    if verbose {
                                        println!("Skipping invalid reading...");
                                    }
                                }
                            }
                        } else {
                            serial_data.push(*c);
                        }
                        // println!("{:?}", &serial_buf[..t]);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                        eprintln!("Broken pipe");
                        ::std::process::exit(1);
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", device, e);
            ::std::process::exit(1);
        }
    }
}
