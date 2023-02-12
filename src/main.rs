use chrono::NaiveDateTime;
// use ndarray::{Array1, arr1, s};
// use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

mod enums;
mod data;
mod errors;
mod temporal;
mod constants;

use data::{get_json_from_file, SpcDataRow};
use temporal::{floor_date, DateMap};
use enums::DateFreq;


fn main() {
    let spc_data = get_json_from_file("data/data.json");
    match spc_data {
        Ok(resp) => {
            let v = &resp.data;
            // let dfreq = &resp.target_date_freq;
            // let dts = v.iter().map(|s| floor_date(s.dt, *dfreq)).collect::<Vec<NaiveDateTime>>();
            let dts = v
                .iter()
                .map(|s: &SpcDataRow| s.dt)
                .collect::<Vec<NaiveDateTime>>();
            let min_date = dts.iter().min().unwrap();
            let max_date = dts.iter().max().unwrap();
            let dmap = DateMap::zeroes(*min_date, *max_date, DateFreq::Year);
            for (k,v) in dmap.0.iter() {
                println!("{}: {:?}", k, v);
            }
        }
        Err(error) => print!("{:?}", error)
    }
}
