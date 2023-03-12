use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug)]
pub struct DateFreqError;

#[derive(Debug)]
pub struct SpcTypeError;

#[derive(Debug)]
pub struct InvalidRowError;

#[derive(Debug, Clone)]
pub enum DataRowError {
    InvalidDateField(String),
    InvalidDecimalField(String),
}

impl Error for DataRowError {}

impl Display for DataRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::InvalidDateField(s) => write!(
                f,
                "Date format '{}' is not valid (must be YYYY-mm-ddTHH:MM:SS)",
                s
            ),
            Self::InvalidDecimalField(s) => write!(f, "Value '{}' is not valid numeric value", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpcDataError {
    InvalidSpcType(String),
    InvalidDateFreq(String),
    InvalidDataRow(Vec<DataRowError>),
}

impl Error for SpcDataError {}

impl Display for SpcDataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::InvalidSpcType(s) => {
                write!(f, "SpcType '{}' is not valid", s)
            }
            Self::InvalidDateFreq(s) => {
                write!(f, "DateFreq '{}' is not valid", s)
            }
            Self::InvalidDataRow(ev) => {
                writeln!(f, "Found {} invalid data rows, showing first 5:", ev.len())?;
                for e in ev {
                    write!(f, " - Invalid data row: {}\n", e)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileLoadError {
    FileNotFound(String),
    InvalidJson,
}

impl Error for FileLoadError {}

impl Display for FileLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::FileNotFound(s) => {
                write!(f, "Could not locate file: {}", s)
            }
            Self::InvalidJson => {
                write!(
                    f,
                    "File contains invalid JSON, or does not match required schema"
                )
            }
        }
    }
}
