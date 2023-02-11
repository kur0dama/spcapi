use spccalc::{Spc, SpcType, DateFreq};
use error::DateFreqError;

mod spcdata;
mod spccalc;
mod error;

fn main() {
    let test_date_freq = "day";
    let date_freq = match &*test_date_freq.to_lowercase() {
        "day" => Ok(DateFreq::Day),
        "week" => Ok(DateFreq::Week),
        "month" => Ok(DateFreq::Month),
        "quarter" => Ok(DateFreq::Quarter),
        "year" => Ok(DateFreq::Year),
        "fiscal_year" => Ok(DateFreq::FiscalYear),
        _ => Err(DateFreqError)
    };
    let spc = Spc {
        spc_type: SpcType::Xbar,
        spc_freq: None,
        spc_data: spcdata::load_csv(
            "data/data.csv",
            "date",
            "failures",
            "sample_size",
        )
    };
    match date_freq {
        Ok(date_freq) => {
            let spc_test = spc.downsample(date_freq);
            print!("{}", spc_test.spc_data);
            print!("{:?}", spc_test.spc_freq.unwrap());
        },
        Err(error) => panic!("{:?}", error),
    }

    
}
