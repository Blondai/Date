//! This module contains the implementation of the [`Month`] enum.

use std::fmt::{self, Display, Formatter};

use crate::{ChronoError, Year};

/// A representation of a month.
///
/// This is a wrapper around `u8`.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
    /// January (Jan)
    January = 1,

    /// February (Feb)
    February = 2,

    /// March (Mar)
    March = 3,

    /// April (Apr)
    April = 4,

    /// May (May)
    May = 5,

    /// June (Jun)
    June = 6,

    /// July (Jul)
    July = 7,

    /// August (Aug)
    August = 8,

    /// September (Sep)
    September = 9,

    /// October (Oct)
    October = 10,

    /// November (Nov)
    November = 11,

    /// December (Dec)
    December = 12,
}

impl Month {
    /// Creates a new [`Month`] instance.
    ///
    /// # Returns
    ///
    /// * `Month` - 1 <= `month` <= 12.
    /// * [`ChronoError::MonthError`] - Otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Month};
    /// // Valid
    /// let month: Month = Month::new(11).unwrap();
    /// assert_eq!(month, Month::November);
    ///
    /// // MonthError
    /// let month_error: ChronoError = Month::new(13).err().unwrap();
    /// assert_eq!(month_error, ChronoError::MonthError(13));
    /// ```
    #[inline]
    pub fn new(number: u8) -> Result<Self, ChronoError> {
        match number {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(ChronoError::MonthError(number)),
        }
    }

    /// Creates a new [`Month`] instance based on a string.
    ///
    /// This can be a string of a number or a string of the written month.
    ///
    /// # Returns
    ///
    /// * [`Month`] - No errors.
    /// * [`ChronoError::ParseError`] - Could not parse `string` as `u8` or could not match to word.
    /// * [`ChronoError::MonthError`] - Something in [`Month::new`] went wrong.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Month};
    /// // String of number
    /// let month: Month = Month::from_string("11").unwrap();
    /// assert_eq!(month, Month::November);
    ///
    /// // String of name
    /// let month: Month = Month::from_string("January").unwrap();
    /// assert_eq!(month, Month::January);
    ///
    /// // string of name short
    /// let month: Month = Month::from_string("Aug").unwrap();
    /// assert_eq!(month, Month::August);
    ///
    /// // MonthError
    /// let month_error: ChronoError = Month::from_string("13").err().unwrap();
    /// assert_eq!(month_error, ChronoError::MonthError(13));
    ///
    /// // ParseError
    /// let parse_error: ChronoError = Month::from_string("First Month").err().unwrap();
    /// assert_eq!(parse_error, ChronoError::ParseError(String::from("First Month")));
    ///
    /// // ParseError
    /// let parse_error: ChronoError = Month::from_string("1000").err().unwrap();
    /// assert_eq!(parse_error, ChronoError::ParseError(String::from("1000")));
    /// ```
    #[inline]
    pub fn from_string(string: &str) -> Result<Self, ChronoError> {
        // Numeric parsing
        if let Ok(number) = string.parse::<u8>() {
            return Month::new(number);
        }

        // String parsing
        match string.to_lowercase().as_str() {
            "january" | "jan" => Ok(Month::January),
            "february" | "feb" => Ok(Month::February),
            "march" | "mar" => Ok(Month::March),
            "april" | "apr" => Ok(Month::April),
            "may" => Ok(Month::May),
            "june" | "jun" => Ok(Month::June),
            "july" | "jul" => Ok(Month::July),
            "august" | "aug" => Ok(Month::August),
            "september" | "sep" => Ok(Month::September),
            "october" | "oct" => Ok(Month::October),
            "november" | "nov" => Ok(Month::November),
            "december" | "dec" => Ok(Month::December),
            _ => Err(ChronoError::ParseError(String::from(string)))?,
        }
    }

    /// Returns the value of the [`Month`] instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Month};
    /// let month: Month = Month::November;
    /// assert_eq!(month.value(), 11);
    /// ```
    #[inline]
    pub const fn value(&self) -> u8 {
        *self as u8
    }

    /// Returns the next month after the current one.
    ///
    /// This will switch over when calling [`Month::next`] on [`Month::December`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{ChronoError, Month};
    /// let month: Month = Month::May;
    /// let next_month: Month = month.next();
    /// assert_eq!(next_month, Month::June);
    ///
    /// let month: Month = Month::December;
    /// let next_month: Month = month.next();
    /// assert_eq!(next_month, Month::January);
    /// ```
    #[inline]
    pub const fn next(&self) -> Self {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }

    /// Adds a number of months to a [`Month`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// # Arguments
    ///
    /// * `years` - The amount of years to add.
    ///
    /// # Returns
    ///
    /// * [`Month`] - No errors.
    /// * [`ChronoError::OverflowError`] - The `months` argument was too large.
    ///
    /// # Notes
    ///
    /// The return of a `Result` is very verbose.
    /// A [`ChronoError::OverflowError`] will only happen, when adding approximately `i32::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{Date, ChronoError, Month};
    /// // Valid addtition
    /// let month: Month = Month::January;
    /// let (new_month, offset): (Month, i32)  = month.add_months(5).unwrap();
    /// assert_eq!(new_month, Month::June);
    /// assert_eq!(offset, 0);
    ///
    /// // Valid addition with offset
    /// let month: Month = Month::December;
    /// let (new_month, offset): (Month, i32)  = month.add_months(1).unwrap();
    /// assert_eq!(new_month, Month::January);
    /// assert_eq!(offset, 1);
    ///
    /// // Valid subtraction with offset
    /// let month: Month = Month::January;
    /// let (new_month, offset): (Month, i32)  = month.add_months(-24).unwrap();
    /// assert_eq!(new_month, Month::January);
    /// assert_eq!(offset, -2);
    ///
    /// // OverflowError
    /// let month: Month = Month::March;
    /// let overflow_error: ChronoError  = month.add_months(i32::MAX).err().unwrap();
    /// assert_eq!(overflow_error, ChronoError::OverflowError);
    /// ```
    #[inline]
    pub fn add_months(&self, months: i32) -> Result<(Month, i32), ChronoError> {
        let current: i32 = *self as i32; // 1–12
        let total: i32 = current
            .checked_add(months)
            .ok_or(ChronoError::OverflowError)?;

        let wrapped: i32 = if total > 0 {
            (total - 1) % 12 + 1
        } else {
            // < 0
            ((12 + (total - 1) % 12) % 12) + 1
        };

        let year_offset: i32 = (total - 1).div_euclid(12);
        let new_month: Month = Month::new(wrapped as u8)?; // Unfailable

        Ok((new_month, year_offset))
    }

    /// Returns the number of days in a month.
    ///
    /// This will also take leap years into account.
    ///
    /// # Examples
    ///
    /// ```
    /// # use date::{Month, Year};
    /// let month: Month = Month::November;
    /// assert_eq!(month.days_in_month(Year::new(2000).unwrap()), 30);
    ///
    /// let month: Month = Month::February;
    /// // Leap year
    /// assert_eq!(month.days_in_month(Year::new(2024).unwrap()), 29);
    /// // Not leap year
    /// assert_eq!(month.days_in_month(Year::new(2025).unwrap()), 28);
    /// ```
    #[inline]
    pub const fn days_in_month(&self, year: Year) -> u8 {
        match self {
            Month::February => {
                if year.is_leap_year() {
                    29_u8
                } else {
                    28_u8
                }
            }
            Month::April | Month::June | Month::September | Month::November => 30_u8,
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => 31_u8,
        }
    }
}

impl Display for Month {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Month::January => "January",
            Month::February => "February",
            Month::March => "March",
            Month::April => "April",
            Month::May => "May",
            Month::June => "June",
            Month::July => "July",
            Month::August => "August",
            Month::September => "September",
            Month::October => "October",
            Month::November => "November",
            Month::December => "December",
        };
        write!(format, "{}", name)
    }
}

impl TryFrom<u8> for Month {
    type Error = ChronoError;

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        Month::new(number)
    }
}

impl TryFrom<i32> for Month {
    type Error = ChronoError;

    fn try_from(number: i32) -> Result<Self, Self::Error> {
        let uint: u8 = number
            .try_into()
            .map_err(|_| ChronoError::ParseError(number.to_string()))?;

        Month::new(uint)
    }
}

impl From<Month> for u8 {
    fn from(month: Month) -> Self {
        month.value()
    }
}

impl From<Month> for i32 {
    fn from(month: Month) -> Self {
        month.value() as i32
    }
}
