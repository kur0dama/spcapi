use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration, Datelike, Weekday};

use crate::enums::DateFreq;

const QTRS_IN_YR: u32 = 4;
const MONS_IN_QTR: u32 = 3;
const QTR_OFFSET: u32 = 1;
const FISCAL_YR_START_MON: u32 = 7;

// pub struct DateRange (NaiveDateTime, NaiveDateTime, Duration);

pub fn floor_date(dt: NaiveDateTime, datepart: DateFreq) -> NaiveDateTime {
    let midnight: NaiveTime = NaiveTime::from_hms_opt(0,0,0).unwrap();
    match datepart {
        DateFreq::Day => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), dt.month(), dt.day()).unwrap(),
            midnight
        ),
        DateFreq::Week => {
            let isowk = dt.iso_week();
            let wk = isowk.week();
            let yr = isowk.year();
            NaiveDateTime::new(
                NaiveDate::from_isoywd_opt(yr, wk, Weekday::Mon).unwrap(),
                midnight
            )
        },
        DateFreq::Month => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), dt.month(), 1).unwrap(),
            midnight
        ),
        DateFreq::Quarter => {
            let qtr = (dt.month() / QTRS_IN_YR * MONS_IN_QTR) + QTR_OFFSET;
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(dt.year(), qtr, 1).unwrap(),
                midnight
            )
        },
        DateFreq::Year => NaiveDateTime::new(
            NaiveDate::from_ymd_opt(dt.year(), 1, 1).unwrap(),
            midnight
        ),
        DateFreq::FiscalYear => {
            let fiscal_year = match dt.month() >= FISCAL_YR_START_MON {
                true => dt.year(),
                false => dt.year() - 1,
            };
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(fiscal_year, FISCAL_YR_START_MON, 1).unwrap(),
                midnight
            )
        },
    }

}