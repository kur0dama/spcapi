use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use chronoutil::RelativeDuration;
use rust_decimal::Decimal;
use std::collections::BTreeMap;

use crate::constants::*;
use crate::data::SpcData;
use crate::enums::DateFreq;
use crate::errors::SpcDataError;

pub fn floor_date(dt: NaiveDateTime, datepart: DateFreq) -> NaiveDateTime {
    // TODO: make func return Result, handle errors in date parsing
    // sort of unnecessary because dates will only validated dates will be passed
    let midnight: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    match datepart {
        DateFreq::Day => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap(),
            midnight,
        ),
        DateFreq::Week => {
            let (wk, yr) = (dt.iso_week().week(), dt.iso_week().year());
            NaiveDateTime::new(
                NaiveDate::from_isoywd_opt(yr, wk, Weekday::Mon).unwrap(),
                midnight,
            )
        }
        DateFreq::Month => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), dt.month(), 1).unwrap(),
            midnight,
        ),
        DateFreq::Quarter => {
            let qtr = (dt.month() / QTRS_IN_YR * MONS_IN_QTR) + QTR_OFFSET;
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(dt.year(), qtr, 1).unwrap(),
                midnight,
            )
        }
        DateFreq::Year => {
            NaiveDateTime::new(NaiveDate::from_ymd_opt(dt.year(), 1, 1).unwrap(), midnight)
        }
        DateFreq::FiscalYear => {
            let fiscal_year = match dt.month() >= FISCAL_YR_START_MON {
                true => dt.year(),
                false => dt.year() - 1,
            };
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(fiscal_year, FISCAL_YR_START_MON, 1).unwrap(),
                midnight,
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateMapVal {
    n: Decimal,
    w: Decimal,
}

impl DateMapVal {
    fn zeroes() -> DateMapVal {
        DateMapVal {
            n: Decimal::new(0, 0),
            w: Decimal::new(0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateMap(pub BTreeMap<NaiveDateTime, DateMapVal>);
impl DateMap {
    pub fn zeroes(start_dt: NaiveDateTime, end_dt: NaiveDateTime, by: DateFreq) -> DateMap {
        let mut date_map = DateMap(BTreeMap::new());
        let mut start_dt_floor = floor_date(start_dt, by);
        let end_dt_floor = floor_date(end_dt, by);
        let dur: RelativeDuration = by.into();

        while start_dt_floor <= end_dt_floor {
            date_map
                .0
                .insert(start_dt_floor, DateMapVal::zeroes());
            start_dt_floor = start_dt_floor + dur;
        }

        date_map
    }
}

impl TryFrom<&SpcData> for DateMap {
    type Error = SpcDataError;
    fn try_from(spc_data: &SpcData) -> Result<Self, SpcDataError> {
        let inner_data = &spc_data.data;
        let date_freq = &spc_data.target_date_freq;
        let date_vec = inner_data
            .iter()
            .map(|s| s.dt)
            .collect::<Vec<NaiveDateTime>>();
        let min_date = date_vec.iter().min().unwrap();
        let max_date = date_vec.iter().max().unwrap();
        let mut date_map = DateMap::zeroes(*min_date, *max_date, *date_freq);
        for record in inner_data.iter() {
            let map_key = floor_date(record.dt, *date_freq);
            let matching_value = date_map.0.get_mut(&map_key);
            match matching_value {
                Some(date_map_val) => {
                    date_map_val.n += record.n;
                    match record.w {
                        Some(w) => date_map_val.w += w,
                        None => (),
                    };
                    ()
                }
                None => (),
            }
        }
        Ok(date_map)
    }
}
