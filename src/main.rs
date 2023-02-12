use chrono::NaiveDateTime;
// use ndarray::{Array1, arr1, s};
// use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

mod enums;
mod data;
mod errors;
mod temporal;

use data::{get_json_from_file};
use temporal::floor_date;
use enums::DateFreq;


fn main() {
    let spc_data = get_json_from_file("data/data.json");
    match spc_data {
        Ok(resp) => {
            let v = &resp.data;
            let dfreq = &resp.target_date_freq;
            let dts = v.iter().map(|s| floor_date(s.dt, *dfreq)).collect::<Vec<NaiveDateTime>>();
            println!("{} to {}", dts.iter().min().unwrap(), dts.iter().max().unwrap());
            for row in v[0..3].to_vec().iter() {
                print!("{:?}, {:?}\n", row, floor_date(row.dt, DateFreq::Week));
            }
        }
        Err(error) => print!("{:?}", error)
    }
}
