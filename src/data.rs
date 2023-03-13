use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::ops::Deref;

use crate::constants::DT_FORMAT;
use crate::enums::JsonDataState;
use crate::enums::*;
use crate::errors::{DataRowError, SpcDataError};

#[derive(Deserialize, Debug, Clone)]
pub struct RequestRow {
    pub dt: String,
    pub n: String,
    pub w: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RequestJson {
    pub spc_type: String,
    pub target_date_freq: String,
    pub data: Vec<RequestRow>,
}

#[derive(Debug, Clone)]
pub struct SpcDataRow {
    pub dt: NaiveDateTime,
    pub n: Decimal,
    pub w: Option<Decimal>,
}

impl TryFrom<&RequestRow> for SpcDataRow {
    type Error = DataRowError;
    fn try_from(row: &RequestRow) -> Result<Self, DataRowError> {
        let dt = NaiveDateTime::parse_from_str(&row.dt, DT_FORMAT);
        let n = Decimal::from_str_exact(&row.n);
        let w = match &row.w {
            Some(x) => {
                let parsed_w = Decimal::from_str_exact(x);
                if parsed_w.is_err() {
                    JsonDataState::PresentInvalid(parsed_w.unwrap_err())
                } else {
                    JsonDataState::PresentValid(parsed_w.unwrap())
                }
            }
            None => JsonDataState::NotPresent,
        };
        match (dt, n, w) {
            (Err(_), _, _) => Err(DataRowError::InvalidDateField(row.dt.to_owned())),
            (_, Err(_), _) => Err(DataRowError::InvalidDecimalField(row.n.to_owned())),
            (_, _, JsonDataState::PresentInvalid(_)) => Err(DataRowError::InvalidDecimalField(
                row.w
                    .as_ref()
                    .unwrap()
                    .to_owned(),
            )),
            (Ok(dt), Ok(n), JsonDataState::PresentValid(w)) => Ok(SpcDataRow {
                dt: dt,
                n: n,
                w: Some(w),
            }),
            (Ok(dt), Ok(n), JsonDataState::NotPresent) => Ok(SpcDataRow {
                dt: dt,
                n: n,
                w: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpcData {
    pub spc_type: SpcType,
    pub target_date_freq: DateFreq,
    pub data: Vec<SpcDataRow>,
}

impl TryFrom<&RequestJson> for SpcData {
    type Error = SpcDataError;
    fn try_from(request: &RequestJson) -> Result<Self, SpcDataError> {
        let spc_type_opt = SpcType::try_from(request.spc_type.deref());
        let target_date_freq_opt = DateFreq::try_from(
            request
                .target_date_freq
                .deref(),
        );
        let data_opt = request
            .data
            .clone()
            .iter()
            .map(|row| SpcDataRow::try_from(row))
            .collect::<Vec<Result<_, _>>>();
        let failed_rows_top5 = data_opt
            .iter()
            .filter(|row| row.is_err())
            .take(5)
            .map(|row| row.clone().unwrap_err())
            .collect::<Vec<DataRowError>>();
        let some_rows_failed: bool = failed_rows_top5.len() > 0;
        match (spc_type_opt, target_date_freq_opt, some_rows_failed) {
            (Err(e), _, _) => Err(e),
            (_, Err(e), _) => Err(e),
            (_, _, true) => Err(SpcDataError::InvalidDataRow(failed_rows_top5)),
            (Ok(spc_type), Ok(target_date_freq), false) => Ok(SpcData {
                spc_type: spc_type,
                target_date_freq: target_date_freq,
                data: data_opt
                    .iter()
                    .map(|row| row.clone().unwrap())
                    .collect::<Vec<SpcDataRow>>(),
            }),
        }
        //
    }
}
