use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use chronoutil::RelativeDuration;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::constants::*;
use crate::enums::DateFreq;

pub fn floor_date(dt: NaiveDateTime, datepart: DateFreq) -> NaiveDateTime {
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
        DateFreq::Year => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), 1, 1).unwrap(),
            midnight,
        ),
        DateFreq::FiscalYear => {
            let fiscal_year = match dt.month() >= FISCAL_YR_START_MON {
                true => dt.year(),
                false => dt.year() - 1,
            };
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(fiscal_year, FISCAL_YR_START_MON, 1)
                    .unwrap(),
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
pub struct DateMap(pub HashMap<NaiveDateTime, DateMapVal>);
impl DateMap {
    pub fn zeroes(
        start_dt: NaiveDateTime,
        end_dt: NaiveDateTime,
        by: DateFreq,
    ) -> DateMap {
        let mut date_map = DateMap(HashMap::new());
        let mut start_dt_floor = floor_date(start_dt, by);
        let end_dt_floor = floor_date(end_dt, by);
        let dur: RelativeDuration = by.into();

        while start_dt_floor <= end_dt_floor {
            date_map.0.insert(start_dt_floor, DateMapVal::zeroes());
            start_dt_floor = start_dt_floor + dur;
        }

        date_map
    }
}
