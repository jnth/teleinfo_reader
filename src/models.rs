use super::schema::teleinfo;
use chrono::prelude::*;
use log::warn;
use regex::Regex;
use std::fmt;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Record {
    id: i32,
    pub dt_utc: NaiveDateTime,
    pub adco: String,
    pub hcjb: i64,
    pub hpjb: i64,
    pub hcjw: i64,
    pub hpjw: i64,
    pub hcjr: i64,
    pub hpjr: i64,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}, dt_utc: {}, adco: {}, hcjb: {}, hpjb: {}, hcjw: {}, hpjw: {}, hcjr: {}, hpjr: {}",
               self.id, self.dt_utc.format("%Y-%m-%d %H:%M:%S"), self.adco, self.hcjb, self.hpjb, self.hcjw, self.hpjw, self.hcjr, self.hpjr)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "teleinfo"]
pub struct NewRecord {
    adco: String,
    hcjb: i64,
    hpjb: i64,
    hcjw: i64,
    hpjw: i64,
    hcjr: i64,
    hpjr: i64,
}

impl NewRecord {
    pub fn from_string(string: String) -> Option<NewRecord> {
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
                let hcjb: i64 = captures
                    .name("hcjb")
                    .expect("Cannot get 'hcjb' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hcjb'");
                let hpjb: i64 = captures
                    .name("hpjb")
                    .expect("Cannot get 'hpjb' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hpjb'");
                let hcjw: i64 = captures
                    .name("hcjw")
                    .expect("Cannot get 'hcjw' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hcjw'");
                let hpjw: i64 = captures
                    .name("hpjw")
                    .expect("Cannot get 'hpjw' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hpjw'");
                let hcjr: i64 = captures
                    .name("hcjr")
                    .expect("Cannot get 'hcjr' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hcjr'");
                let hpjr: i64 = captures
                    .name("hpjr")
                    .expect("Cannot get 'hpjr' value with the regex")
                    .as_str()
                    .parse::<i64>()
                    .expect("Invalid value of 'hpjr'");

                Some(NewRecord {
                    adco,
                    hcjb,
                    hpjb,
                    hcjw,
                    hpjw,
                    hcjr,
                    hpjr,
                })
            }
            None => {
                warn!("Cannot captures data with the regex");
                None
            }
        }
    }
}
