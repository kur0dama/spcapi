// use std::error::Error;
// use std::io;
// use std::process;

// use serde::Deserialize;
// use polars::prelude::*;
// use chrono::{DateTime, Utc};

use spccalc::{Spc, SpcType, DateFreq};

mod spcdata;
mod spccalc;

fn main() {
    let spc = Spc {
        spc_type: SpcType::Xbar,
        spc_freq: None,
        spc_data: spcdata::load_csv(
            &"data/data.csv",
            &"date",
            &"failures",
            &"sample_size",
        )
    };
    let spc_test = spc.downsample(DateFreq::Week);
    print!("{}", spc_test.spc_data);
    print!("{:?}", spc_test.spc_freq.unwrap());
    
}
