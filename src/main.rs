use chrono::NaiveDateTime;
use std::path::Path;

mod constants;
mod data;
mod enums;
mod errors;
mod temporal;

use data::{get_json_from_file, SpcData, SpcDataRow};
use temporal::DateMap;

fn main() {
    let request_json = get_json_from_file(Path::new("data/data.json")).unwrap();
    let spc_data_opt = SpcData::try_from(&request_json);
    match spc_data_opt {
        Ok(spc_data) => {
            let inner_data = &spc_data.data;
            let date_freq = &spc_data.target_date_freq;
            let date_vec = inner_data
                .iter()
                .map(|s: &SpcDataRow| s.dt)
                .collect::<Vec<NaiveDateTime>>();
            let min_date = date_vec.iter().min().unwrap();
            let max_date = date_vec.iter().max().unwrap();
            let dmap = DateMap::zeroes(*min_date, *max_date, *date_freq);
            for (k, v) in dmap.0.iter() {
                println!("{}: {:?}", k, v);
            }
        }
        // Err(error) => print!("{}", error),
        Err(error) => println!("{}", error),
    }
}
