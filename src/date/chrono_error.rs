//! This module contains the implementation of the [`ChronoError`] enum.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[allow(unused_imports)]
use crate::Date;
use crate::{Age, Year};

/// An enum for handling any errors involved in the creation of [`Date`]s or calculation of [`Age`]s
#[derive(Debug, Clone, PartialEq)]
pub enum ChronoError {
    /// Year was outside plausible range.
    YearError(i32),

    /// Month does not exist.
    MonthError(u8),

    /// Month does not have provided amount of days.
    DayError { day: u8, days_in_month: u8 },

    /// Person is too old ore too young.
    AgeError(u8),

    /// Could not parse string into a given format.
    ParseError(String),

    /// Over-/Underflow in addition/subtraction.
    OverflowError,
}

impl Display for ChronoError {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChronoError::YearError(year) => write!(
                format,
                "Year Error: {} is not between {} and {}",
                year,
                Year::MIN,
                Year::MAX
            ),
            ChronoError::MonthError(month) => {
                write!(format, "Month Error: {} is not a valid month", month)
            }
            ChronoError::DayError { day, days_in_month } => write!(
                format,
                "Day Error: month has {} days, not {}",
                day, days_in_month
            ),
            ChronoError::AgeError(age) => write!(
                format,
                "Age Error: {} not between {} and {}",
                age,
                Age::MIN,
                Age::MAX
            ),
            ChronoError::ParseError(string) => write!(format, "Parse Error: {}", string),
            ChronoError::OverflowError => write!(format, "Overflow Error"),
        }
    }
}

impl Error for ChronoError {}
