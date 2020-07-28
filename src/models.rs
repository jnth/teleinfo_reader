use log::warn;
use std::time::SystemTime;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    dt: SystemTime,
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
    pub fn from_string(string: String) -> Option<Record> {
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
                    dt: SystemTime::now(),
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

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Cannot serialize into JSON string")
    }
}
