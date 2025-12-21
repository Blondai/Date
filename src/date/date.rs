//! This module contains the implementation of the [`Date`] struct.

use crate::{Age, ChronoError, Day, Month, Rounding, Year};
use std::fmt::{self, Display, Formatter};
use std::ops::Add;

/// A representation of a [`Date`].
///
/// This is based on [`Year`], [`Month`] and [`Day`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The year.
    year: Year,

    /// The month.
    month: Month,

    /// The day.
    day: Day,
}

impl Date {
    /// Creates a new [`Date`] instance.
    ///
    /// Nothing can go wrong because of the type safety of [`Year`], [`Month`] and [`Day`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, Day, Month, Year};
    /// let year: Year = Year::new(2024).unwrap();
    /// let month: Month = Month::January;
    /// let day: Day = Day::new(1, month, year).unwrap();
    /// let date: Date = Date::new(year, month, day);
    /// ```
    #[inline]
    pub fn new(year: Year, month: Month, day: Day) -> Self {
        Self { year, month, day }
    }

    /// Creates a new [`Date`] instance based on numbers.
    ///
    /// This calls [`Year::new`], [`Month::new`] and [`Day::new`].
    ///
    /// # Errors
    ///
    /// * [`ChronoError::YearError`] - The `year` is not between [`Year::MIN`] and [`Year::MAX`] both included.
    /// * [`ChronoError::MonthError`] - The `month` is not inside the interval [1, 12].
    /// * [`ChronoError::DayError`] - The `month` of the `year` does not have the amount of days provided.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{ChronoError, Date, Year};
    /// // Valid
    /// let date: Date = Date::new_num(2024, 1, 1).unwrap();
    ///
    /// // YearError
    /// let year_error: ChronoError = Date::new_num(Year::MAX + 1, 1, 1).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(Year::MAX + 1));
    ///
    /// // MonthError
    /// let month_error: ChronoError = Date::new_num(2024, 13, 1).err().unwrap();
    /// assert_eq!(month_error, ChronoError::MonthError(13));
    ///
    /// // DayError
    /// let day_error: ChronoError = Date::new_num(2023, 2, 29).err().unwrap();
    /// assert_eq!(day_error, ChronoError::DayError { day: 29, days_in_month: 28 });
    /// ```
    #[inline]
    pub fn new_num(year: i32, month: u8, day: u8) -> Result<Self, ChronoError> {
        let year: Year = Year::new(year)?;
        let month: Month = Month::new(month)?;
        let day: Day = Day::new(day, month, year)?;

        Ok(Self { year, month, day })
    }

    /// Creates a new [`Date`] instance based on numbers.
    ///
    /// This calls [`Year::new_const`], [`Month::new_const`] and [`Day::new_const`].
    ///
    /// # Panics
    ///
    /// The `month` of the `year` does not have the amount of days provided.
    /// The `month` is not between 1 (january) and 12 (december).
    /// The `year` is not between [`Year::MIN`] and [`Year::MAX`] both included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// const DATE: Date = Date::new_const(2024, 1, 1);
    /// assert_eq!(DATE.day().value(), 1);
    /// assert_eq!(DATE.month().value(), 1);
    /// assert_eq!(DATE.year().value(), 2024);
    /// ```
    #[inline]
    pub const fn new_const(year: i32, month: u8, day: u8) -> Self {
        let day: Day = Day::new_const(day, month, year);
        let month: Month = Month::new_const(month);
        let year: Year = Year::new_const(year);

        Self { year, month, day }
    }

    /// Creates a new [`Date`] instance the string 'ddmmyyyy'.
    ///
    /// This calls the appropriate `new` methods of [`Year`], [`Month`] and [`Day`].
    ///
    /// # Errors
    ///
    /// * [`ChronoError::ParseError`] - Could not parse any part as a number.
    /// This could also happen the string length is not equal to 8.
    /// * [`ChronoError::YearError`] - The `year` is not between [`Year::MIN`] and [`Year::MAX`] both included.
    /// * [`ChronoError::MonthError`] - The `month` is not inside the interval [1, 12].
    /// * [`ChronoError::DayError`] - The `month` of the `year` does not have the amount of days provided.
    ///
    /// # Notes
    ///
    /// This method could probably enhanced by automatically splitting the string at any '.' or '/'
    /// and automatically recognizing if it is 'ddmmyyyy' or 'yyyy.mm.dd'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{ChronoError, Date, Year};
    /// // Valid
    /// let date: Date = Date::from_string("01012024").unwrap();
    /// assert_eq!(date, Date::new_num(2024, 1, 1).unwrap());
    ///
    /// // ParseError (Too short)
    /// let year_error: ChronoError = Date::from_string("112024").err().unwrap();
    /// assert_eq!(year_error, ChronoError::ParseError(String::from("112024")));
    ///
    /// // ParseError (Wrong symbols)
    /// let month_error: ChronoError = Date::from_string(" 1 12024").err().unwrap();
    /// assert_eq!(month_error, ChronoError::ParseError(String::from("day ' 1'")));
    ///
    /// // DayError
    /// let day_error: ChronoError = Date::from_string("29022023").err().unwrap();
    /// assert_eq!(day_error, ChronoError::DayError { day: 29, days_in_month: 28 });
    /// ```
    pub fn from_string(string: &str) -> Result<Self, ChronoError> {
        if string.len() != 8 {
            return Err(ChronoError::ParseError(String::from(string)));
        }

        // String slices
        let day_str: &str = &string[0..2];
        let month_str: &str = &string[2..4];
        let year_str: &str = &string[4..8];

        // Converted to numbers
        let day_u8: u8 = day_str
            .parse()
            .map_err(|_| ChronoError::ParseError(format!("day '{}'", day_str)))?;
        let month_u8: u8 = month_str
            .parse()
            .map_err(|_| ChronoError::ParseError(format!("month '{}'", month_str)))?;
        let year_i32: i32 = year_str
            .parse()
            .map_err(|_| ChronoError::ParseError(format!("year '{}'", year_str)))?;

        // Converted to own types
        let year: Year = Year::new(year_i32)?;
        let month: Month = Month::new(month_u8)?;
        let day: Day = Day::new(day_u8, month, year)?;

        Ok(Self { year, month, day })
    }

    /// Returns the value of the [`Year`] attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 1, 20).unwrap();
    /// assert_eq!(date.year().value(), 2024);
    /// ```
    #[inline]
    pub const fn year(&self) -> Year {
        self.year
    }

    /// Returns the value of the [`Month`] attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 1, 20).unwrap();
    /// assert_eq!(date.month().value(), 1);
    /// ```
    #[inline]
    pub const fn month(&self) -> Month {
        self.month
    }

    /// Returns the value of the [`Day`] attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 2, 20).unwrap();
    /// assert_eq!(date.day().value(), 20);
    /// ```
    #[inline]
    pub const fn day(&self) -> Day {
        self.day
    }

    /// Returns a new [`Date`] with the `day` set to 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 1, 20).unwrap();
    /// assert_eq!(date.begin_of_month(), Date::new_num(2024, 1, 1).unwrap());
    /// ```
    #[inline]
    pub fn begin_of_month(&self) -> Self {
        let month: Month = self.month;
        let year: Year = self.year;
        let day: Day = Day::new_unchecked(1_u8); // safe

        Self { year, month, day }
    }

    /// Returns a new [`Date`] with the `day` set to the end of month.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 1, 20).unwrap();
    /// assert_eq!(date.end_of_month(), Date::new_num(2024, 1, 31).unwrap());
    /// ```
    #[inline]
    pub fn end_of_month(&self) -> Self {
        let month: Month = self.month;
        let year: Year = self.year;
        let day: Day = Day::new_unchecked(month.days_in_month(year)); // safe

        Self { year, month, day }
    }

    /// Returns a new [`Date`] with the `day` set to the mid of the month.
    ///
    /// This is 14 for [`Month::February`] and 15 otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 1, 20).unwrap();
    /// assert_eq!(date.mid_of_month(), Date::new_num(2024, 1, 15).unwrap());
    /// let date: Date = Date::new_num(2024, 2, 1).unwrap();
    /// assert_eq!(date.mid_of_month(), Date::new_num(2024, 2, 14).unwrap());
    /// ```
    #[inline]
    pub fn mid_of_month(&self) -> Self {
        let month: Month = self.month;
        let year: Year = self.year;
        let day_u8: u8 = match month {
            Month::February => 14_u8,
            _ => 15_u8,
        };
        let day: Day = Day::new_unchecked(day_u8); // safe

        Self { year, month, day }
    }

    /// Returns the string representation "dd.mm.yyyy" of the `Date` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// assert_eq!(date.format_dmy(), String::from("01.06.2024"));
    /// ```
    #[inline]
    pub fn format_dmy(&self) -> String {
        format!(
            "{:02}.{:02}.{}",
            self.day.value(),
            self.month as u8,
            self.year.value()
        )
    }

    /// Returns the string representation "yyyy.mm.dd" of the `Date` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// assert_eq!(date.format_ymd(), String::from("2024.06.01"));
    /// ```
    #[inline]
    pub fn format_ymd(&self) -> String {
        format!(
            "{}.{:02}.{:02}",
            self.year.value(),
            self.month as u8,
            self.day.value()
        )
    }

    /// Adds a number of years to a [`Date`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// This uses the [`Year::add_years`] method.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::YearError`] - The resulting year is not between [`Year::MIN`] and [`Year::MAX`].
    /// * [`ChronoError::OverflowError`] - The resulting year is larger than [`i32::MAX`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{ChronoError, Date};
    /// // Valid addition
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// let new_date: Date = date.add_years(6).unwrap();
    /// assert_eq!(new_date, Date::new_num(2030, 6, 1).unwrap());
    ///
    /// // Valid subtraction
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// let new_date: Date = date.add_years(-4).unwrap();
    /// assert_eq!(new_date, Date::new_num(2020, 6, 1).unwrap());
    ///
    /// // YearError
    /// let date: Date = Date::new_num(2095, 6, 1).unwrap();
    /// let year_error: ChronoError = date.add_years(10).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(2105));
    ///
    /// // OverflowError
    /// let date: Date = Date::new_num(2000, 12, 31).unwrap();
    /// let overflow_error: ChronoError = date.add_years(i32::MAX).err().unwrap();
    /// assert_eq!(overflow_error, ChronoError::OverflowError);
    /// ```
    pub fn add_years(&self, years: i32) -> Result<Self, ChronoError> {
        let new_year: Year = self.year.add_years(years)?;

        Ok(Self {
            year: new_year,
            month: self.month,
            day: self.day,
        })
    }

    /// Adds a number of months to a [`Date`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// This uses the [`Month::add_months`] method.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::YearError`] - The resulting year is not between [`Year::MIN`] and [`Year::MAX`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{ChronoError, Date, Year};
    /// // Valid addition
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// let new_date: Date = date.add_months(1).unwrap();
    /// assert_eq!(new_date, Date::new_num(2024, 7, 1).unwrap());
    /// let new_date: Date = date.add_months(7).unwrap();
    /// assert_eq!(new_date, Date::new_num(2025, 1, 1).unwrap());
    ///
    /// // Valid subtraction
    /// let date: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let new_date: Date = date.add_months(-1).unwrap();
    /// assert_eq!(new_date, Date::new_num(2024, 11, 30).unwrap());
    ///
    /// // YearError
    /// let date: Date = Date::new_num(Year::MAX, 6, 1).unwrap();
    /// let year_error: ChronoError = date.add_months(10).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(Year::MAX + 1));
    /// ```
    pub fn add_months(&self, months: i32) -> Result<Self, ChronoError> {
        let (new_month, year_offset): (Month, i32) = self.month.add_months(months)?;
        let new_year: Year = self.year.add_years(year_offset)?;

        // Clamp day if necessary
        let max_day: u8 = new_month.days_in_month(new_year);
        let day_u8: u8 = self.day.value().min(max_day);
        let new_day: Day = Day::new(day_u8, new_month, new_year)?;

        Ok(Self {
            year: new_year,
            month: new_month,
            day: new_day,
        })
    }

    /// Adds a number of days to a [`Date`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::YearError`] - Based on [`Date::add_months`] and [`Date::add_years`].
    ///
    /// # Notes
    ///
    /// This method could probably be speed up dramatically using formulas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{ChronoError, Date, Year};
    /// // Valid addition
    /// let date: Date = Date::new_num(2024, 6, 1).unwrap();
    /// let new_date: Date = date.add_days(1).unwrap();
    /// assert_eq!(new_date, Date::new_num(2024, 6, 2).unwrap());
    ///
    /// let date: Date = Date::new_num(2024, 12, 30).unwrap();
    /// let new_date: Date = date.add_days(2).unwrap();
    /// assert_eq!(new_date, Date::new_num(2025, 1, 1).unwrap());
    ///
    /// // Valid subtraction
    /// let date: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let new_date: Date = date.add_days(-1).unwrap();
    /// assert_eq!(new_date, Date::new_num(2024, 12, 30).unwrap());
    ///
    /// // YearError
    /// let date: Date = Date::new_num(Year::MAX, 12, 30).unwrap();
    /// let year_error: ChronoError = date.add_days(10).err().unwrap();
    /// assert_eq!(year_error, ChronoError::YearError(Year::MAX + 1));
    /// ```
    pub fn add_days(&self, days: i32) -> Result<Self, ChronoError> {
        let mut year: Year = self.year;
        let mut month: Month = self.month;
        let mut day: i32 = self.day.value() as i32;

        let mut remaining: i32 = days;

        // Add or subtract days one month at a time
        while remaining != 0 {
            let days_in_current_month: i32 = month.days_in_month(year) as i32;

            if remaining > 0 {
                // Add
                let days_left_in_month: i32 = days_in_current_month - day;

                if remaining > days_left_in_month {
                    remaining -= days_left_in_month + 1;
                    day = 1;
                    let (next_month, year_offset): (Month, i32) = month.add_months(1)?;
                    month = next_month;
                    year = year.add_years(year_offset)?;
                } else {
                    day += remaining;
                    remaining = 0;
                }
            } else {
                // Subtract
                if day + remaining > 0 {
                    day += remaining;
                    remaining = 0;
                } else {
                    let (prev_month, year_offset): (Month, i32) = month.add_months(-1)?;
                    month = prev_month;
                    year = year.add_years(year_offset)?;
                    let days_in_prev: i32 = month.days_in_month(year) as i32;
                    remaining += day;
                    day = days_in_prev;
                }
            }
        }

        let day: Day = Day::new(day as u8, month, year)?;

        Ok(Self { year, month, day })
    }

    /// Returns the number of days since 00.01.0000.
    ///
    /// This method is formula-based and leap-year safe.
    #[inline]
    fn to_days(&self) -> i32 {
        let full_years: i32 = self.year.value() - 1;

        // Days in previous full years with leaps
        let mut days: i32 = full_years * 365 + full_years / 4 - full_years / 100 + full_years / 400;

        // Cumulative days in months (non-leap by default)
        const MONTH_DAYS: [i32; 13] = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        days += MONTH_DAYS[self.month as usize];

        // Add current day
        days += i32::from(self.day);

        // Leap year adjustment
        if self.month > Month::February && self.year.is_leap_year() {
            days += 1;
        }

        days
    }

    /// Calculates the difference in days between two [`Date`]s.
    ///
    /// This is always a positive number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Date;
    /// let date_1: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.day_difference(&date_2), 0);
    /// assert_eq!(date_2.day_difference(&date_1), 0);
    ///
    /// let date_1: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 20).unwrap();
    /// assert_eq!(date_1.day_difference(&date_2), 11);
    ///
    /// let date_1: Date = Date::new_num(2004, 6, 12).unwrap();
    /// let date_2: Date = Date::new_num(2001, 5, 9).unwrap();
    /// assert_eq!(date_1.day_difference(&date_2), 1130);
    /// ```
    #[inline]
    pub fn day_difference(&self, other: &Date) -> i32 {
        (self.to_days() - other.to_days()).abs()
    }

    /// Calculates the difference in full months between two [`Date`]s.
    ///
    /// This is always a positive number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, Rounding};
    /// let date_1: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.month_difference(&date_2, Rounding::Floor), 0);
    ///
    /// let date_1: Date = Date::new_num(2024, 10, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.month_difference(&date_2, Rounding::Floor), 2);
    ///
    /// let date_1: Date = Date::new_num(2024, 10, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 5).unwrap();
    /// assert_eq!(date_1.month_difference(&date_2, Rounding::Floor), 1);
    /// ```
    pub fn month_difference(&self, other: &Date, rounding: Rounding) -> i32 {
        // Sorts `Date`s correctly
        let (first, last): (&Date, &Date) = if self < other {
            (self, other)
        } else {
            (other, self)
        };

        let mut floor_diff: i32 = (last.year.value() - first.year.value()) * 12_i32
            + (last.month as i32 - first.month as i32);

        // Fixes month_difference(31.03.2004, 30.04.2004) != 1
        let first_is_eom: bool = first.day.value() == first.month.days_in_month(first.year);
        let last_is_eom: bool = last.day.value() == last.month.days_in_month(last.year);
        if !(first_is_eom && last_is_eom) && last.day < first.day {
            floor_diff -= 1_i32;
        }

        match rounding {
            Rounding::Floor => floor_diff,
            Rounding::Ceil => {
                if first.day != last.day {
                    floor_diff + 1
                } else {
                    floor_diff
                }
            }
            Rounding::Nearest => {
                let day_difference: i32 = (i32::from(first.day) - i32::from(last.day)) % 30;
                if day_difference >= 15 {
                    floor_diff + 1
                } else {
                    floor_diff
                }
            }
        }
    }

    /// Calculates the difference in full years between two [`Date`]s.
    ///
    /// This is always a positive number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, Rounding};
    /// let date_1: Date = Date::new_num(2024, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.year_difference(&date_2, Rounding::Floor), 0);
    ///
    /// let date_1: Date = Date::new_num(2024, 3, 31).unwrap();
    /// let date_2: Date = Date::new_num(2025, 3, 30).unwrap();
    /// assert_eq!(date_1.year_difference(&date_2, Rounding::Floor), 0);
    ///
    /// let date_1: Date = Date::new_num(2024, 6, 12).unwrap();
    /// let date_2: Date = Date::new_num(2020, 1, 30).unwrap();
    /// assert_eq!(date_1.year_difference(&date_2, Rounding::Floor), 4);
    /// ```
    pub fn year_difference(&self, other: &Date, rounding: Rounding) -> i32 {
        // Sorts `Date`s correctly
        let (first, last): (&Date, &Date) = if self < other {
            (self, other)
        } else {
            (other, self)
        };

        let mut floor_diff: i32 = last.year.value() - first.year.value();

        // Lexicographical comparison
        if (last.month, last.day) < (first.month, first.day) {
            floor_diff -= 1_i32;
        }

        match rounding {
            Rounding::Floor => floor_diff,
            Rounding::Ceil => {
                if (first.month, first.day) != (last.month, last.day) {
                    floor_diff + 1
                } else {
                    floor_diff
                }
            }
            Rounding::Nearest => {
                let month_diff: i32 = (last.month as i32 - first.month as i32 + 12) % 12;

                if month_diff > 6 {
                    // More than 6 months
                    floor_diff + 1
                } else if month_diff < 6 {
                    // Less than 6 months
                    floor_diff
                } else {
                    // Exactly 6 months past
                    let day_diff: i32 = last.day.value() as i32 - first.day.value() as i32;
                    if day_diff >= 0 {
                        // Exactly 6 months or more
                        floor_diff + 1
                    } else {
                        // Less than 6 months
                        floor_diff
                    }
                }
            }
        }
    }

    /// Calculates the actuarial [`Age`] of a person.
    ///
    /// This is calculated by getting the effective date plus six month and calculating the [`Date::year_difference`].
    ///
    /// # Errors
    ///
    /// * [`ChronoError::AgeError`] - The resulting age would be outside the range of [`Age::MIN`] and [`Age::MAX`].
    ///
    /// ```rust
    /// # use date::Date;
    /// let date_1: Date = Date::new_num(1959, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.actuarial_age(&date_2).unwrap().value(), 65);
    ///
    /// let date_1: Date = Date::new_num(2001, 11, 5).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.actuarial_age(&date_2).unwrap().value(), 23);
    ///
    /// let date_1: Date = Date::new_num(1959, 2, 12).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.actuarial_age(&date_2).unwrap().value(), 66);
    ///
    /// let date_1: Date = Date::new_num(2001, 11, 5).unwrap();
    /// let date_2: Date = Date::new_num(2025, 6, 30).unwrap();
    /// assert_eq!(date_1.actuarial_age(&date_2).unwrap().value(), 24);
    ///
    /// let date_1: Date = Date::new_num(1965, 7, 1).unwrap();
    /// let date_2: Date = Date::new_num(2025, 6, 30).unwrap();
    /// assert_eq!(date_1.actuarial_age(&date_2).unwrap().value(), 60);
    /// ```
    pub fn actuarial_age(&self, effective_date: &Date) -> Result<Age, ChronoError> {
        let effective_effective_date: Date = effective_date.add_months(6_i32)?;

        // Fixes 01.07. problem
        if self.day.value() == 1_u8
            && self.month.value() == (effective_effective_date.month.value() + 1_u8) % 12_u8
        {
            Age::try_from(
                self.year_difference(&effective_effective_date.end_of_month(), Rounding::Floor)
                    + 1_i32,
            )
        } else {
            Age::try_from(
                self.year_difference(&effective_effective_date.end_of_month(), Rounding::Floor),
            )
        }
    }

    /// Calculates the civil [`Age`] of a person.
    ///
    /// This is calculated using [`Date::year_difference`].
    ///
    /// # Errors
    ///
    /// * [`ChronoError::AgeError`] - The resulting age would be outside the range of [`Age::MIN`] and [`Age::MAX`].
    ///
    /// ```rust
    /// # use date::Date;
    /// let date_1: Date = Date::new_num(1959, 12, 31).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.civil_age(&date_2).unwrap().value(), 65);
    ///
    /// let date_1: Date = Date::new_num(2001, 11, 5).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.civil_age(&date_2).unwrap().value(), 23);
    ///
    /// let date_1: Date = Date::new_num(1959, 2, 12).unwrap();
    /// let date_2: Date = Date::new_num(2024, 12, 31).unwrap();
    /// assert_eq!(date_1.civil_age(&date_2).unwrap().value(), 65);
    ///
    /// let date_1: Date = Date::new_num(2001, 11, 5).unwrap();
    /// let date_2: Date = Date::new_num(2025, 6, 30).unwrap();
    /// assert_eq!(date_1.civil_age(&date_2).unwrap().value(), 23);
    ///
    /// let date_1: Date = Date::new_num(1965, 7, 1).unwrap();
    /// let date_2: Date = Date::new_num(2025, 6, 30).unwrap();
    /// assert_eq!(date_1.civil_age(&date_2).unwrap().value(), 59);
    /// ```
    pub fn civil_age(&self, effective_date: &Date) -> Result<Age, ChronoError> {
        Age::try_from(self.year_difference(effective_date, Rounding::Floor))
    }
}

impl Display for Date {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "{}", self.format_dmy())
    }
}

impl From<Year> for String {
    fn from(year: Year) -> String {
        format!("{}", year)
    }
}

impl Add<i32> for Date {
    type Output = Date;

    /// [`Add`]s a specific amount of `days` to a [`Date`].
    ///
    /// Use negative numbers for subtraction.
    ///
    /// # Panics
    ///
    /// Any error in [`Date::add_days`] will cause this method to panic.
    fn add(self, days: i32) -> Self::Output {
        self.add_days(days).unwrap()
    }
}
