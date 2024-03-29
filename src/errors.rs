use crate::constants;
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

#[derive(Debug, Clone)]
pub enum DataRowError {
    InvalidDateField(String),
    InvalidDecimalField(String),
    ZeroDenominatorField,
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
            Self::ZeroDenominatorField => write!(
                f,
                "'w' field has zero value; if provided, weights should be non-zero"
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpcDataError {
    InvalidSpcType(String),
    InvalidDateFreq(String),
    InvalidDataRows(Vec<(usize, DataRowError)>),
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
            Self::InvalidDataRows(ev) => {
                writeln!(
                    f,
                    "Found {} invalid data rows, showing first {}:",
                    ev.len(),
                    constants::NUM_ROW_ERRORS_DISP
                )?;
                for (i, e) in ev {
                    write!(f, " - [Row {}] {}\n", i, e)?;
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
