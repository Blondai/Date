//! This module contains the implementation of the [`Accuracy`] struct.

use std::fmt::{self, Display, Formatter};

#[allow(unused_imports)]
use crate::{Date, RataTemporis};

/// The [`Accuracy`] in the calculation of the [`RataTemporis`] calculations.
///
/// The [`Default`] value is [`Accuracy::MonthExact`].
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Accuracy {
    /// Uses the [`Date::day_difference`] method.
    DayExact,

    /// Uses the [`Date::month_difference`] method.
    MonthExact,

    /// Uses the [`Date::year_difference`] method.
    YearExact,
}

impl Default for Accuracy {
    #[inline]
    fn default() -> Accuracy {
        Accuracy::MonthExact
    }
}

impl Display for Accuracy {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Accuracy::DayExact => write!(format, "Day-exact"),
            Accuracy::MonthExact => write!(format, "Month-exact"),
            Accuracy::YearExact => write!(format, "Year-exact"),
        }
    }
}
