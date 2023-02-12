// SPC calculations based on arbitrary data
//

use polars::prelude::*;

#[derive(Debug)]
pub struct DateFreqError;






impl SpcType {

}

#[derive(Debug, Clone)]
pub struct Spc {
    pub spc_type: SpcType,
    pub spc_freq: Option<DateFreq>,
    pub spc_data: DataFrame,
}


impl Spc {

    pub fn downsample(self: &Self, date_freq: DateFreq) -> Spc {

        match (&self.spc_freq, date_freq) {
            (Some(DateFreq::Year), DateFreq::Month) => panic!("Cannot upsample from year to month"),
            (Some(DateFreq::Year), DateFreq::Day) => panic!("Cannot upsample from year to day"),
            (Some(DateFreq::Month), DateFreq::Day) => panic!("Cannot upsample from month to day"),
            _ => {
                let resample_options: DynamicGroupOptions = match date_freq {
                    DateFreq::Day => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("1d"),
                        period: Duration::parse("1d"), 
                        offset: Duration::parse("0s"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::WindowBound,
                    },
                    DateFreq::Week => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("1w"),
                        period: Duration::parse("1w"), 
                        offset: Duration::parse("0s"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::Monday,
                    },
                    DateFreq::Month => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("1mo"),
                        period: Duration::parse("1mo"), 
                        offset: Duration::parse("0s"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::WindowBound,
                    },
                    DateFreq::Quarter => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("3mo"),
                        period: Duration::parse("3mo"), 
                        offset: Duration::parse("0s"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::WindowBound,
                    },
                    DateFreq::Year => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("1y"),
                        period: Duration::parse("1y"), 
                        offset: Duration::parse("0s"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::WindowBound,
                    },
                    DateFreq::FiscalYear => DynamicGroupOptions {
                        index_column: "dt".into(), 
                        every: Duration::parse("1y"),
                        period: Duration::parse("1y"), 
                        offset: Duration::parse("6mo"), 
                        truncate: false, 
                        include_boundaries: false, 
                        closed_window: ClosedWindow::Left,
                        start_by: StartBy::WindowBound,
                    },
                };
                let resampled_data: DataFrame = self.spc_data
                    .clone()
                    .lazy()
                    .groupby_dynamic(vec![], resample_options)
                    .agg([
                        col("n").sum(),
                        col("w").sum()
                    ])
                    .collect()
                    .unwrap();

                Spc { spc_type: self.spc_type, spc_freq: Some(date_freq), spc_data: resampled_data }
            }
        }
    }
}