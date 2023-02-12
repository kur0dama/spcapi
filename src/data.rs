use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::BufReader;
use std::str::FromStr;
use serde::Deserialize;
use rust_decimal::Decimal;
// use rust_decimal_macros::dec;
use chrono::NaiveDateTime;
// use serde_json::Result;

use crate::enums::*;
// use crate::errors::*;

const DT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

#[derive(Deserialize, Debug, Clone)]
pub struct RequestRow {
    pub dt: String,
    pub n: String,
    pub w: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SpcDataRow {
    pub dt: NaiveDateTime,
    pub n: Decimal,
    pub w: Option<Decimal>,
}

impl RequestRow {

    pub fn try_into_typed_struct(self: &Self) -> Result<SpcDataRow, Box<dyn Error>> {
        let dt = NaiveDateTime::parse_from_str(&self.dt, DT_FORMAT)?;
        let n = Decimal::from_str(&self.n)?;
        let w = match &self.w {
            Some(x) => Some(Decimal::from_str(&x).unwrap()),
            None => None,
        };
        let outstruct = SpcDataRow {
            dt: dt,
            n: n,
            w: w,
        };
        Ok(outstruct)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RequestJson {
    pub spc_type: String,
    pub target_date_freq: String,
    pub data: Vec<RequestRow>,
}

#[derive(Debug, Clone)]
pub struct SpcData {
    pub spc_type: SpcType,
    pub target_date_freq: DateFreq,
    pub data: Vec<SpcDataRow>,
}

impl RequestJson {

    pub fn try_into_typed_struct(self: &Self) -> Result<SpcData, Box<dyn Error>> {
        let spc_type = SpcType::try_from(&*self.spc_type).expect("Invalid SpcType");
        let target_date_freq = DateFreq::try_from(&*self.target_date_freq).expect("Invalid DateFreq");
        let data = &self
            .data
            .clone()
            .iter()
            .map(|r: &RequestRow| r.clone().try_into_typed_struct().unwrap())
            .collect::<Vec<SpcDataRow>>();
        Ok(
            SpcData {
                spc_type: spc_type,
                target_date_freq: target_date_freq,
                data: data.to_vec(),
            }
        )
    }
}

pub fn get_json_from_file<P: AsRef<Path>>(path: P) -> Result<SpcData, Box<dyn Error>> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let resp: RequestJson = serde_json::from_reader(reader)?;

    Ok(resp.try_into_typed_struct().unwrap())
}