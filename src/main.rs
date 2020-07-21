use serialport::prelude::*;
use std::time::Duration;
use std::io;
use regex::Regex;
use serde_json;
use clap::{App, Arg};
use log::debug;

#[macro_use] extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    adco: String,
    optarif: String,
    isousc: u8,
    hchc: u64,
    hchp: u64,
    ptec: String,
    iinst: u8,
    imax: u8,
    papp: u16,
    hhphc: String,
    motdetat: String,
}

impl Record {
    fn from_string(string: String) -> Record {
        let re = Regex::new(concat!(
            r"\x0aADCO (?P<adco>\d+) .\x0d",
            r"\x0aOPTARIF (?P<optarif>.+) .\x0d",
            r"\x0aISOUSC (?P<isousc>\d+) .\x0d",
            r"\x0aHCHC (?P<hchc>\d+) .\x0d",
            r"\x0aHCHP (?P<hchp>\d+) .\x0d",
            r"\x0aPTEC (?P<ptec>.+) .\x0d",
            r"\x0aIINST (?P<iinst>\d+) .\x0d",
            r"\x0aIMAX (?P<imax>\d+) .\x0d",
            r"\x0aPAPP (?P<papp>\d+) .\x0d",
            r"\x0aHHPHC (?P<hhphc>.+) .\x0d",
            r"\x0aMOTDETAT (?P<motdetat>.+) .\x0d",
        )).unwrap();
        let captures = re.captures(string.as_str()).unwrap();

        let adco = captures.name("adco").unwrap().as_str().to_owned();
        let optarif = captures.name("optarif").unwrap().as_str().to_owned();
        let isousc: u8 = captures.name("isousc").unwrap().as_str().parse::<u8>().unwrap();
        let hchc: u64 = captures.name("hchc").unwrap().as_str().parse::<u64>().unwrap();
        let hchp: u64 = captures.name("hchp").unwrap().as_str().parse::<u64>().unwrap();
        let ptec = captures.name("ptec").unwrap().as_str().to_owned();
        let iinst: u8 = captures.name("iinst").unwrap().as_str().parse::<u8>().unwrap();
        let imax: u8 = captures.name("imax").unwrap().as_str().parse::<u8>().unwrap();
        let papp: u16 = captures.name("papp").unwrap().as_str().parse::<u16>().unwrap();
        let hhphc = captures.name("hhphc").unwrap().as_str().to_owned();
        let motdetat = captures.name("motdetat").unwrap().as_str().to_owned();

        Record { adco, optarif, isousc, hchc, hchp, ptec, iinst, imax, papp, hhphc, motdetat }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
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
                .index(1))
        .arg(
            Arg::with_name("verbose")
                .help("Verbose mode")
                .short("v")
                .takes_value(false))
        .get_matches();

    let device = matches.value_of("device").unwrap();
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
                        }
                        else if c == &3 && started {
                            debug!("get end of record character");
                            let record: Record = Record::from_string(String::from_utf8_lossy(&serial_data).into_owned());
                            if verbose {
                                println!("get record: {:?}", record);
                            }
                            // println!("{}", record.to_json());
                        }
                        else {
                            serial_data.push(*c);
                        }
                        // println!("{:?}", &serial_buf[..t]);
                    },
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
