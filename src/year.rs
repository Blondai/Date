//! This module contains the implementation of the [`Year`] struct.

use std::fmt::{self, Display, Formatter};

use crate::ChronoError;

/// A representation of a year.
///
/// This is a wrapper around `i32`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year {
    /// The year
    year: i32,
}

impl Year {
    /// Creates a new [`Year`] instance.
    ///
    /// # Returns
    ///
    /// * [`Year`] - [`Year::MIN`] <= `year` <= [`Year::MAX`].
    /// * [`ChronoError::YearError`] - Otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Year};
    /// // Valid
    /// let year: Year = Year::new(2025).unwrap();
    /// assert_eq!(year.value(), 2025);
    ///
    /// // YearError
    /// let year_error: ChronoError = Year::new(Year::MAX + 1).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(Year::MAX + 1));
    /// ```
    #[inline]
    pub fn new(year: i32) -> Result<Self, ChronoError> {
        if year >= Self::MIN && year <= Self::MAX {
            Ok(Self { year })
        } else {
            Err(ChronoError::YearError(year))
        }
    }

    /// Returns a new [`Year`] instance without any checks.
    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn new_unchecked(year: i32) -> Self {
        Self { year }
    }

    /// Creates a new [`Year`] instance based on a string.
    ///
    /// # Returns
    ///
    /// * [`Year`] - No errors.
    /// * [`ChronoError::YearError`] - Something in [`Year::new`] went wrong.
    /// * [`ChronoError::ParseError`] - Could not parse `string` as `i32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use date::{Date, ChronoError, Year};
    /// // Valid
    /// let year: Year = Year::from_string("2025").unwrap();
    /// assert_eq!(year.value(), 2025);
    ///
    /// // YearError
    /// let year_error: ChronoError = Year::from_string("0").err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(0));
    ///
    /// // ParseError
    /// let parse_error: ChronoError = Year::from_string("Twenty Twenty-Five").err().unwrap();
    /// assert_eq!(parse_error, ChronoError::ParseError(String::from("Twenty Twenty-Five")));
    /// ```
    #[inline]
    pub fn from_string(string: &str) -> Result<Self, ChronoError> {
        let year: i32 = string
            .parse()
            .map_err(|_| ChronoError::ParseError(String::from(string)))?;

        Self::new(year)
    }

    /// Returns the value of the [`Year`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::Year;
    /// let year: Year = Year::new(2025).unwrap();
    /// assert_eq!(year.value(), 2025);
    /// ```
    #[inline]
    pub const fn value(&self) -> i32 {
        self.year
    }

    /// Checks if a year is a leap year.
    ///
    /// When year % 4 = 0 and year % 100 != 0 or year % 400 = 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Year};
    /// // Not a leap year
    /// let year: Year = Year::new(2025).unwrap();
    /// assert!(!year.is_leap_year());
    ///
    /// // Leap year
    /// let year: Year = Year::new(2024).unwrap();
    /// assert!(year.is_leap_year());
    ///
    /// // Leap year
    /// let year: Year = Year::new(2000).unwrap();
    /// assert!(year.is_leap_year());
    /// ```
    #[inline]
    pub const fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0
    }

    /// Returns the number of days in a year.
    ///
    /// A leap year has 366 days and any other year has 365.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Year};
    /// // Not a leap year
    /// let year: Year = Year::new(2025).unwrap();
    /// assert_eq!(year.days_in_year(), 365);
    ///
    /// // Leap year
    /// let year: Year = Year::new(2024).unwrap();
    /// assert_eq!(year.days_in_year(), 366);
    ///
    /// // Leap year
    /// let year: Year = Year::new(2000).unwrap();
    /// assert_eq!(year.days_in_year(), 366);
    /// ```
    #[inline]
    pub const fn days_in_year(&self) -> i32 {
        if self.is_leap_year() {
            366_i32
        } else {
            365_i32
        }
    }

    /// Adds a number of years to a [`Year`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// # Arguments
    ///
    /// * `years` - The amount of years to add.
    ///
    /// # Returns
    ///
    /// * [`Year`] - No errors.
    /// * [`ChronoError::YearError`] - Something in [`Year::new`] went wrong.
    /// * [`ChronoError::OverflowError`] - The `years` argument was too large.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Year};
    /// // Valid addition
    /// let year: Year = Year::new(2025).unwrap();
    /// let new_year: Year = year.add_years(10).unwrap();
    /// assert_eq!(new_year.value(), 2035);
    ///
    /// // Valid subtraction
    /// let year: Year = Year::new(2000).unwrap();
    /// let new_year: Year = year.add_years(-20).unwrap();
    /// assert_eq!(new_year.value(), 1980);
    ///
    /// // YearError
    /// let year: Year = Year::new(2095).unwrap();
    /// let year_error: ChronoError = year.add_years(10).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(2105));
    ///
    /// // OverflowError
    /// let year: Year = Year::new(2000).unwrap();
    /// let overflow_error: ChronoError = year.add_years(i32::MAX).err().unwrap();
    /// assert_eq!(overflow_error, ChronoError::OverflowError);
    /// ```
    #[inline]
    pub fn add_years(&self, years: i32) -> Result<Self, ChronoError> {
        let new_year: i32 = self
            .year
            .checked_add(years)
            .ok_or(ChronoError::OverflowError)?;

        Self::new(new_year)
    }

    /// The smallest reasonable year supported.
    ///
    /// This is just set to easily find mistakes when handling dates of birth.
    /// All methods should still work for any [`Year::MIN`]
    pub const MIN: i32 = 1900_i32;

    /// The largest reasonable year supported.
    ///
    /// This is just set to easily find mistakes when handling dates of birth.
    /// All methods should still work for any [`Year::MAX`].
    pub const MAX: i32 = 2100_i32;
}

impl Display for Year {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.year)
    }
}

impl TryFrom<i32> for Year {
    type Error = ChronoError;

    fn try_from(year: i32) -> Result<Self, Self::Error> {
        Self::new(year)
    }
}

impl From<Year> for i32 {
    fn from(year: Year) -> Self {
        year.value()
    }
}
