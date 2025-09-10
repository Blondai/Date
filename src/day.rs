//! This module contains the implementation of the [`Day`] struct.

use std::fmt::{self, Display, Formatter};

use crate::{ChronoError, Month, Year};

/// A representation of a day in a month.
///
/// This is a wrapper around `u8`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day {
    /// The day
    day: u8,
}

impl Day {
    /// Creates a new [`Day`] instance based on a string.
    ///
    /// # Returns
    ///
    /// * [`Day`] - No errors.
    /// * [`ChronoError::DayError`] - Month does not have the amount of days provided.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Day, Month, Year};
    /// // Valid
    /// let day: Day = Day::new(31, Month::December, Year::new(2025).unwrap()).unwrap();
    /// assert_eq!(day.value(), 31);
    ///
    /// // Leap year
    /// let day: Day = Day::new(29, Month::February, Year::new(2024).unwrap()).unwrap();
    /// assert_eq!(day.value(), 29);
    ///
    /// // DayError
    /// let day_error: ChronoError = Day::new(29, Month::February, Year::new(2025).unwrap()).err().unwrap();
    /// assert_eq!(day_error, ChronoError::DayError { day: 29, days_in_month: 28 });
    /// ```
    #[inline]
    pub fn new(day: u8, month: Month, year: Year) -> Result<Self, ChronoError> {
        let days_in_month: u8 = month.days_in_month(year);

        if day >= 1_u8 && day <= days_in_month {
            Ok(Self { day })
        } else {
            Err(ChronoError::DayError { day, days_in_month })
        }
    }

    /// Returns a new [`Day`] instance without any checks.
    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn new_unchecked(day: u8) -> Self {
        Self { day }
    }

    /// Returns the value of the [`Day`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Day, Month, Year};
    /// let day: Day = Day::new(31, Month::December, Year::new(2025).unwrap()).unwrap();
    /// assert_eq!(day.value(), 31);
    /// ```
    #[inline]
    pub const fn value(&self) -> u8 {
        self.day
    }
}

impl Display for Day {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.day)
    }
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        day.value()
    }
}

impl From<Day> for i32 {
    fn from(day: Day) -> Self {
        day.value() as i32
    }
}
