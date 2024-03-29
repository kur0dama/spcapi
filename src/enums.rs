use chronoutil::RelativeDuration;

use crate::errors::SpcDataError;

#[derive(Debug, Copy, Clone)]
pub enum SpcType {
    Xbar, // sample average
    Xmr,  // X moving range
    P,    // proportion
    C,    // counts
    U,    // rate
    Mr,   // moving range
    I,    // individuals
    G,    // cases between events
    T,    // time between events
}

impl TryFrom<&str> for SpcType {
    type Error = SpcDataError;

    fn try_from(value: &str) -> Result<Self, SpcDataError> {
        match value {
            "xbar" => Ok(SpcType::Xbar),
            "xmr" => Ok(SpcType::Xmr),
            "p" => Ok(SpcType::P),
            "c" => Ok(SpcType::C),
            "u" => Ok(SpcType::U),
            "mr" => Ok(SpcType::Mr),
            "i" => Ok(SpcType::I),
            "g" => Ok(SpcType::G),
            "t" => Ok(SpcType::T),
            _ => Err(SpcDataError::InvalidSpcType(value.into())),
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
    type Error = SpcDataError;

    fn try_from(value: &str) -> Result<Self, SpcDataError> {
        match value {
            "day" => Ok(DateFreq::Day),
            "week" => Ok(DateFreq::Week),
            "month" => Ok(DateFreq::Month),
            "quarter" => Ok(DateFreq::Quarter),
            "year" => Ok(DateFreq::Year),
            "fiscal_year" => Ok(DateFreq::FiscalYear),
            _ => Err(SpcDataError::InvalidDateFreq(value.into())),
        }
    }
}

impl Into<RelativeDuration> for DateFreq {
    fn into(self) -> RelativeDuration {
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

#[derive(Debug, Clone)]
pub enum JsonDataState<T, E> {
    PresentValid(T),
    PresentInvalid(E),
    NotPresent,
}
