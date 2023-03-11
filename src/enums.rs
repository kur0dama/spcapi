use chronoutil::RelativeDuration;

use crate::errors::*;

#[derive(Debug, Copy, Clone)]
pub enum SpcType {
    Xbar, // sample average
    P,    // proportion
    C,    // counts
    U,    // rate
    MR,   // moving range
    I,    // individuals
    G,    // cases between events
    T,    // time between events
}

impl TryFrom<&str> for SpcType {
    type Error = SpcTypeError;

    fn try_from(value: &str) -> Result<Self, SpcTypeError> {
        match value {
            "xbar" => Ok(SpcType::Xbar),
            "p" => Ok(SpcType::P),
            "c" => Ok(SpcType::C),
            "u" => Ok(SpcType::U),
            "mr" => Ok(SpcType::MR),
            "i" => Ok(SpcType::I),
            "g" => Ok(SpcType::G),
            "t" => Ok(SpcType::T),
            _ => Err(SpcTypeError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DateFreq {
    Day,
    Week,
    Month,
    Quarter,
    Year,
    FiscalYear,
}

impl TryFrom<&str> for DateFreq {
    type Error = DateFreqError;

    fn try_from(value: &str) -> Result<Self, DateFreqError> {
        match value {
            "day" => Ok(DateFreq::Day),
            "week" => Ok(DateFreq::Week),
            "month" => Ok(DateFreq::Month),
            "quarter" => Ok(DateFreq::Quarter),
            "year" => Ok(DateFreq::Year),
            "fiscal_year" => Ok(DateFreq::FiscalYear),
            _ => Err(DateFreqError),
        }
    }
}

impl DateFreq {
    pub fn get_assoc_duration(self: &Self) -> RelativeDuration {
        match self {
            DateFreq::Day => RelativeDuration::days(1),
            DateFreq::Week => RelativeDuration::days(7),
            DateFreq::Month => RelativeDuration::months(1),
            DateFreq::Quarter => RelativeDuration::months(3),
            DateFreq::Year => RelativeDuration::months(12),
            DateFreq::FiscalYear => RelativeDuration::months(12),
        }
    }
}
