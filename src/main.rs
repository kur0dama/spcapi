use chrono::Datelike;
// use ndarray::{Array1, arr1, s};
// use rust_decimal::Decimal;
// use rust_decimal_macros::dec;

mod enums;
mod data;
mod errors;

use data::{get_json_from_file};


fn main() {
    let spc_data = get_json_from_file("data/data.json");
    match spc_data {
        Ok(resp) => {
            let v = &resp.data;
            let row = v.first().unwrap().clone();
            print!("{:?}, {:?}", row, row.dt.weekday());
        }
        Err(error) => print!("{:?}", error)
    }
}
