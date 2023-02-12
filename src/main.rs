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
            let date_freq = DateFreq::try_from(resp.target_date_freq).unwrap();
            let date_vec = v
                .iter()
                .map(|s: &SpcDataRow| s.dt)
                .collect::<Vec<NaiveDateTime>>();
            let min_date = date_vec.iter().min().unwrap();
            let max_date = date_vec.iter().max().unwrap();
            let dmap = DateMap::zeroes(*min_date, *max_date, date_freq);
            for (k,v) in dmap.0.iter() {
                println!("{}: {:?}", k, v);
            }
        }
        Err(error) => print!("{:?}", error)
    }
}
