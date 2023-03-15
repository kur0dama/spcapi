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
    let date_map_opt = DateMap::try_from(&spc_data_opt.unwrap());
    match date_map_opt {
        Ok(date_map) => {
            for (k, v) in date_map.0.iter() {
                println!("{}: {:?}", k, v);
            }
        }
        Err(e) => println!("{}", e),
    }
}
