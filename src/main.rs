use clap::{App, Arg};
use log::{debug, warn};
use regex::Regex;
use serde_json;
use serialport::prelude::*;
use std::io;
use std::time::Duration;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    adco: String,
    optarif: String,
    isousc: u8,
    hcjb: u64,
    hpjb: u64,
    hcjw: u64,
    hpjw: u64,
    hcjr: u64,
    hpjr: u64,
    ptec: String,
    demain: String,
    iinst: u8,
    imax: u8,
    papp: u16,
    hhphc: String,
    motdetat: String,
}

impl Record {
    fn from_string(string: String) -> Option<Record> {
        let re = Regex::new(concat!(
            r"\x0aADCO (?P<adco>\d+) .\x0d",
            r"\x0aOPTARIF (?P<optarif>.+) .\x0d",
            r"\x0aISOUSC (?P<isousc>\d+) .\x0d",
            r"\x0aBBRHCJB (?P<hcjb>\d+) .\x0d",
            r"\x0aBBRHPJB (?P<hpjb>\d+) .\x0d",
            r"\x0aBBRHCJW (?P<hcjw>\d+) .\x0d",
            r"\x0aBBRHPJW (?P<hpjw>\d+) .\x0d",
            r"\x0aBBRHCJR (?P<hcjr>\d+) .\x0d",
            r"\x0aBBRHPJR (?P<hpjr>\d+) .\x0d",
            r"\x0aPTEC (?P<ptec>.+) .\x0d",
            r"\x0aDEMAIN (?P<demain>.+) .\x0d",
            r"\x0aIINST (?P<iinst>\d+) .\x0d",
            r"\x0aIMAX (?P<imax>\d+) .\x0d",
            r"\x0aPAPP (?P<papp>\d+) .\x0d",
            r"\x0aHHPHC (?P<hhphc>.+) .\x0d",
            r"\x0aMOTDETAT (?P<motdetat>.+) .\x0d",
        ))
        .expect("Invalid regex");
        match re.captures(string.as_str()) {
            Some(captures) => {
                let adco = captures
                    .name("adco")
                    .expect("Cannot get 'adco' value with the regex")
                    .as_str()
                    .to_owned();
                let optarif = captures
                    .name("optarif")
                    .expect("Cannot get 'optarif' value with the regex")
                    .as_str()
                    .to_owned();
                let isousc: u8 = captures
                    .name("isousc")
                    .expect("Cannot get 'isousc' value with the regex")
                    .as_str()
                    .parse::<u8>()
                    .expect("Invalid value of 'isousc'");
                let hcjb: u64 = captures
                    .name("hcjb")
                    .expect("Cannot get 'hcjb' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hcjb'");
                let hpjb: u64 = captures
                    .name("hpjb")
                    .expect("Cannot get 'hpjb' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hpjb'");
                let hcjw: u64 = captures
                    .name("hcjw")
                    .expect("Cannot get 'hcjw' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hcjw'");
                let hpjw: u64 = captures
                    .name("hpjw")
                    .expect("Cannot get 'hpjw' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hpjw'");
                let hcjr: u64 = captures
                    .name("hcjr")
                    .expect("Cannot get 'hcjr' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hcjr'");
                let hpjr: u64 = captures
                    .name("hpjr")
                    .expect("Cannot get 'hpjr' value with the regex")
                    .as_str()
                    .parse::<u64>()
                    .expect("Invalid value of 'hpjr'");
                let ptec = captures
                    .name("ptec")
                    .expect("Cannot get 'ptec' value with the regex")
                    .as_str()
                    .to_owned();
                let demain = captures
                    .name("demain")
                    .expect("Cannot get 'demain' value with the regex")
                    .as_str()
                    .to_owned();
                let iinst: u8 = captures
                    .name("iinst")
                    .expect("Cannot get 'iinst' value with the regex")
                    .as_str()
                    .parse::<u8>()
                    .expect("Invalid value of 'iinst'");
                let imax: u8 = captures
                    .name("imax")
                    .expect("Cannot get 'imax' value with the regex")
                    .as_str()
                    .parse::<u8>()
                    .expect("Invalid value of 'imax'");
                let papp: u16 = captures
                    .name("papp")
                    .expect("Cannot get 'papp' value with the regex")
                    .as_str()
                    .parse::<u16>()
                    .expect("Invalid value of 'papp'");
                let hhphc = captures
                    .name("hhphc")
                    .expect("Cannot get 'hhphc' value with the regex")
                    .as_str()
                    .to_owned();
                let motdetat = captures
                    .name("motdetat")
                    .expect("Cannot get 'motdetat' value with the regex")
                    .as_str()
                    .to_owned();

                Some(Record {
                    adco,
                    optarif,
                    isousc,
                    hcjb,
                    hpjb,
                    hcjw,
                    hpjw,
                    hcjr,
                    hpjr,
                    ptec,
                    demain,
                    iinst,
                    imax,
                    papp,
                    hhphc,
                    motdetat,
                })

            }
            None => {
                warn!("Cannot captures data with the regex");
                None
            }
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Cannot serialize into JSON string")
    }
}

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
                            debug!("get start character of a record");
                            serial_data.clear();
                            started = true;
                        } else if c == &3 && started {
                            debug!("get end of record character");
                            match Record::from_string(String::from_utf8_lossy(&serial_data).into_owned()) {
                                Some(record) => {
                                    if verbose {
                                        println!("get record: {:?}", record);
                                    }
                                }
                                None => {
                                    if verbose {
                                        println!("skipping invalid reading...");
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
