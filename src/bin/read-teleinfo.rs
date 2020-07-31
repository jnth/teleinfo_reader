use clap::{App, Arg};
use log::debug;
use serialport::prelude::*;
use std::io;
use std::time::Duration;
use teleinfo_reader::{establish_connection, save_record_into_db};
use teleinfo_reader::models::NewRecord;

fn main() {
    // Arguments and options
    let matches = App::new("Teleinfo Reader")
        .version("0.1.0")
        .author("Jonathan Virga <jonathan.virga@gmail.com>")
        .about("Read teleinfomation data from serial device")
        .arg(
            Arg::with_name("device")
                .help("Path of serial device")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Verbose mode")
                .short("v")
                .takes_value(false),
        )
        .get_matches();

    let device = matches
        .value_of("device")
        .expect("Cannot read 'device' parameter from arguments");
    let verbose = matches.is_present("verbose");
    let baud_rate = 1200;

    let conn = establish_connection();

    let settings = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Seven,
        flow_control: FlowControl::None,
        parity: Parity::Even,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };
    let port = serialport::open_with_settings(device, &settings);
    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1];
            let mut serial_data: Vec<u8> = Vec::new();
            let mut started: bool = false;

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
                            match NewRecord::from_string(String::from_utf8_lossy(&serial_data).into_owned()) {
                                Some(new_record) => {
                                    let record = save_record_into_db(&conn, new_record);
                                    if verbose {
                                        println!("Get record: {:?}", record);
                                    }
                                }
                                None => {
                                    if verbose {
                                        println!("Skipping invalid reading...");
                                    }
                                }
                            }

                        // println!("{}", record.to_json());
                        } else {
                            serial_data.push(*c);
                        }
                        // println!("{:?}", &serial_buf[..t]);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                        eprintln!("Broken pipe");
                        ::std::process::exit(1);
                    },
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
