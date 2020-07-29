use rand::Rng;
use serialport::prelude::*;
use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;

const CHARSET: &[u8] = b"0123456789";

fn generate_record(adco: &str) -> String {
    let kvs = vec![
        ("ADCO", adco),
        ("OPTARIF", "BBR("),
        ("ISOUSC", "60"),
        ("BBRHCJB", "000130857"),
        ("BBRHPJB", "000272039"),
        ("BBRHCJW", "000000000"),
        ("BBRHPJW", "000000000"),
        ("BBRHCJR", "000000000"),
        ("BBRHPJR", "000000000"),
        ("PTEC", "HPJB"),
        ("DEMAIN", "----"),
        ("IINST", "003"),
        ("IMAX", "090"),
        ("PAPP", "00770"),
        ("HHPHC", "A"),
        ("MOTDETAT", "000000"),
    ];

    let mut record: String = String::new();
    record.push('\x02'); // start text (STX)
    for (k, v) in kvs.iter() {
        record.push_str(&format!("\x0a{} {} X\x0d", k, v)) // the checksum is not valid here!
    }
    record.push('\x03'); // end text (ETX)

    record
}

fn main() {
    let device = "/tmp/input";
    let baud_rate = 1200;

    // Random value of ADCO
    let mut rng = rand::thread_rng();
    let adco: String = (0..12)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    // Run `socat` command to open two connected file sockets
    println!("Run socat command to open two connected file sockets...");
    let _socat = Command::new("socat")
        .args(&[
            "-d",
            "-d",
            &format!("pty,link={},raw,echo=0", device),
            "pty,link=/tmp/output,raw,echo=0",
        ])
        .spawn()
        .expect("Failed to start socat command");
    std::thread::sleep(Duration::from_secs(2));

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
            println!("Writing data to {} at {} baud", &device, &baud_rate);
            loop {
                let record = generate_record(&adco);
                for c in record.chars() {
                    match port.write(c.to_string().as_bytes()) {
                        Ok(_) => {
                            print!("{}", c);
                            std::io::stdout().flush().unwrap();
                        }
                        Err(ref e)
                            if e.kind() == io::ErrorKind::TimedOut
                                || e.kind() == io::ErrorKind::BrokenPipe =>
                        {
                            ()
                        }
                        Err(e) => eprintln!("{:?}", e),
                    }
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", device, e);
            ::std::process::exit(1);
        }
    }
}
