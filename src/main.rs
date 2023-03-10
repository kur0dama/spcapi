use chrono::NaiveDateTime;
use std::path::Path;

mod constants;
mod data;
mod enums;
mod errors;
mod load;
mod temporal;

use data::SpcData;
use load::get_json_from_file;
use temporal::DateMap;

fn main() {
    let request_json_opt = get_json_from_file(Path::new("data/data.json"));
    if request_json_opt.is_err() {
        println!("{}", request_json_opt.unwrap_err());
        return;
    }
    let spc_data_opt = SpcData::try_from(&request_json_opt.unwrap());
    match spc_data_opt {
        Ok(spc_data) => {
            let inner_data = &spc_data.data;
            let date_freq = &spc_data.target_date_freq;
            let date_vec = inner_data
                .iter()
                .map(|s| s.dt)
                .collect::<Vec<NaiveDateTime>>();
            let min_date = date_vec.iter().min().unwrap();
            let max_date = date_vec.iter().max().unwrap();
            let dmap = DateMap::zeroes(*min_date, *max_date, *date_freq);
            for (k, v) in dmap.0.iter() {
                println!("{}: {:?}", k, v);
            }
        }
        Err(e) => println!("{}", e),
    }
}
