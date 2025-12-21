use std::fmt::{Display, Formatter};

#[allow(unused_imports)]
use crate::{Date, RataTemporis};

/// Specifies the [`Rounding`] strategy for difference calculations.
///
/// This is used in [`Date::month_difference`] and [`Date::year_difference`] and
/// therefore [Date::actuarial_age], [`Date::civil_age`].
/// Furthermore, it is used in all methods of the [`RataTemporis`] struct.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rounding {
    /// Rounds to the nearest whole unit, with halves rounds up.
    Nearest,

    /// Rounds down to the nearest whole unit.
    Floor,

    /// Rounds up to the nearest whole unit.
    Ceil,
}

impl Default for Rounding {
    fn default() -> Self {
        Rounding::Nearest
    }
}

impl Display for Rounding {
    fn fmt(&self, format: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rounding::Nearest => write!(format, "Nearest"),
            Rounding::Floor => write!(format, "Floor"),
            Rounding::Ceil => write!(format, "Ceil"),
        }
    }
}
