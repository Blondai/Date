//! This module contains the implementation of the [`RataTemporis`] struct and its [`RataTemporisError`].

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{Accuracy, Date, PensionAge, Rounding};
#[allow(unused_imports)]
use crate::{ChronoError, PensionMonths, PensionYears};

/// Handles the calculation of the [`RataTemporis`].
///
/// This is based on the §2 of the german ["Gesetz zur Verbesserung der betrieblichen Altersversorgung"](https://www.gesetze-im-internet.de/betravg/__2.html).
/// It is defined as the actual service time divided by the possible service time up to the pension age.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RataTemporis {
    /// The date of birth.
    birth_date: Date,

    /// The date of entry.
    entry_date: Date,

    /// The date of exit.
    exit_date: Date,
}

impl RataTemporis {
    /// Creates a new [`RataTemporis`] instance.
    ///
    /// # Errors
    ///
    /// * [`RataTemporisError::WrongOrder`] - `birth_date` > `entry_date` or `exit_date` > `entry_date`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use date::{Date, RataTemporis, RataTemporisError};
    /// // Valid
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    ///
    /// // Birth after entry
    /// let birth_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let wrong_order: RataTemporisError = RataTemporis::new(birth_date, entry_date, exit_date).err().unwrap();
    /// assert_eq!(wrong_order, RataTemporisError::WrongOrder { first_date: birth_date, second_date: entry_date });
    ///
    /// // Entry after exit
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2025, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2020, 12, 31).unwrap();
    /// let wrong_order: RataTemporisError = RataTemporis::new(birth_date, entry_date, exit_date).err().unwrap();
    /// assert_eq!(wrong_order, RataTemporisError::WrongOrder { first_date: entry_date, second_date: exit_date });
    /// ```
    #[inline]
    pub fn new(
        birth_date: Date,
        entry_date: Date,
        exit_date: Date,
    ) -> Result<Self, RataTemporisError> {
        RataTemporisError::check_order(&birth_date, &entry_date)?;
        RataTemporisError::check_order(&entry_date, &exit_date)?;

        Ok(Self {
            birth_date,
            entry_date,
            exit_date,
        })
    }

    /// Returns the actual service time (m) based on a given [`Accuracy`] and [`Rounding`].
    ///
    /// This is the time between the `entry_date` and the `exit_date`.
    ///
    /// # Errors
    ///
    /// * [`RataTemporisError::NegativeDifference`] - The difference between `entry_date` and `exit_date` is negative.
    /// This should be prevented by the [`RataTemporis::new`] method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    ///
    /// // MonthExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 71);
    ///
    /// // DayExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 2_191);
    ///
    /// // YearExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 5);
    ///
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap().add_days(1).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    ///
    /// // MonthExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 72);
    ///
    /// // DayExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 2_192);
    ///
    /// // YearExact
    /// let m: u32 = rata_temporis.actual_service(Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 6);
    /// ```
    #[inline]
    pub fn actual_service(
        &self,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<u32, RataTemporisError> {
        // Can not be negative
        let m: i32 = match accuracy {
            Accuracy::DayExact => self.entry_date.day_difference(&self.exit_date),
            Accuracy::MonthExact => self.entry_date.month_difference(&self.exit_date, rounding),
            Accuracy::YearExact => self.entry_date.year_difference(&self.exit_date, rounding),
        };

        let m: u32 = m
            .try_into()
            .map_err(|_| RataTemporisError::NegativeDifference)?;

        Ok(m)
    }

    /// Returns the possible service (n) based on a given [`Accuracy`] and [`Rounding`].
    ///
    /// This is the time between the `entry_date` and the `pension_date`.
    /// The `pension_date` is the date [`PensionYears`] years and [`PensionMonths`] months after the `birth_date`.
    ///
    /// # Errors
    ///
    /// * [`RataTemporisError::WrongOrder`] - The `entry_date` is after the `pension_date`.
    /// * [`RataTemporisError::YearError`] - The addition of `pension_years` went wrong.
    /// * [`RataTemporisError::MonthError`] - The addition of `pension_months` went wrong.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, PensionAge, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    /// let pension_age: PensionAge = PensionAge::just_65();
    ///
    /// // MonthExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 540);
    ///
    /// // DayExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 16_437);
    ///
    /// // YearExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 45);
    ///
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap().add_days(-1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    /// let pension_age: PensionAge = PensionAge::just_65();
    ///
    /// // MonthExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 539);
    ///
    /// // DayExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 16_436);
    ///
    /// // YearExact
    /// let n: u32 = rata_temporis.possible_service(pension_age, Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 44);
    /// ```
    pub fn possible_service(
        &self,
        pension_age: PensionAge,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<u32, RataTemporisError> {
        let pension_years: i32 = i32::from(pension_age.pension_years());
        let pension_months: i32 = i32::from(pension_age.pension_months());

        let pension_date: Date = self
            .birth_date
            .add_years(pension_years)
            .map_err(|_| RataTemporisError::YearError { pension_years })?
            .add_months(pension_months)
            .map_err(|_| RataTemporisError::MonthError { pension_months })?;

        RataTemporisError::check_order(&self.entry_date, &pension_date)?;

        // Can not be negative
        let n: i32 = match accuracy {
            Accuracy::DayExact => self.entry_date.day_difference(&pension_date),
            Accuracy::MonthExact => self.entry_date.month_difference(&pension_date, rounding),
            Accuracy::YearExact => self.entry_date.year_difference(&pension_date, rounding),
        };

        let n: u32 = n
            .try_into()
            .map_err(|_| RataTemporisError::NegativeDifference)?;

        Ok(n)
    }

    /// Returns the possible service (n) based on the legal [`PensionAge`].
    ///
    /// The [`PensionAge`] is automatically calculated based on the birthyear using [`PensionAge::from_birthyear`].
    ///
    /// # Errors
    ///
    /// See [`RataTemporis::possible_service`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, PensionAge, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    ///
    /// // MonthExact
    /// let n: u32 = rata_temporis.possible_service_birthyear(Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 564);
    ///
    /// // DayExact
    /// let n: u32 = rata_temporis.possible_service_birthyear(Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 17_167);
    ///
    /// // YearExact
    /// let n: u32 = rata_temporis.possible_service_birthyear(Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(n, 47);
    /// ```
    #[inline]
    pub fn possible_service_birthyear(
        &self,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<u32, RataTemporisError> {
        let pension_age: PensionAge = PensionAge::from_birthyear(self.birth_date.year());

        self.possible_service(pension_age, accuracy, rounding)
    }

    /// Returns the pair consisting of actual service (m) and possible service (n).
    ///
    /// This uses the [`RataTemporis::actual_service`] and [`RataTemporis::possible_service`] methods.
    ///
    /// # Errors
    ///
    /// See [`RataTemporis::actual_service`] and [`RataTemporis::possible_service`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, PensionAge, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    /// let pension_age: PensionAge = PensionAge::just_65();
    ///
    /// // MonthExact
    /// let (m, n): (u32, u32) = rata_temporis.rata_temporis_pair(pension_age, Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 71);
    /// assert_eq!(n, 540);
    ///
    /// // DayExact
    /// let (m, n): (u32, u32) = rata_temporis.rata_temporis_pair(pension_age, Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 2_191);
    /// assert_eq!(n, 16_437);
    ///
    /// // YearExact
    /// let (m, n): (u32, u32) = rata_temporis.rata_temporis_pair(pension_age, Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert_eq!(m, 5);
    /// assert_eq!(n, 45);
    /// ```
    #[inline]
    pub fn rata_temporis_pair(
        &self,
        pension_age: PensionAge,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<(u32, u32), RataTemporisError> {
        let m: u32 = self.actual_service(accuracy, rounding)?;
        let n: u32 = self.possible_service(pension_age, accuracy, rounding)?;

        Ok((m, n))
    }

    /// Returns the [`RataTemporis`] as actual service (m) divided by possible service (n).
    ///
    /// This uses the [`RataTemporis::actual_service`] and [`RataTemporis::possible_service`] methods.
    ///
    /// If the [`RataTemporis::possible_service`] is zero the rata temporis is also zero, as no service is possible.
    ///
    /// # Errors
    ///
    /// See [`RataTemporis::actual_service`] and [`RataTemporis::possible_service`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, PensionAge, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    /// let pension_age: PensionAge = PensionAge::just_65();
    ///
    /// // MonthExact
    /// let rata: f64 = rata_temporis.rata_temporis(pension_age, Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert!((rata - 71. / 540.).abs() < f64::EPSILON);
    ///
    /// // DayExact
    /// let rata: f64 = rata_temporis.rata_temporis(pension_age, Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert!((rata - 2_191. / 16_437.).abs() < f64::EPSILON);
    ///
    /// // YearExact
    /// let rata: f64 = rata_temporis.rata_temporis(pension_age, Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert!((rata - 5. / 45.).abs() < f64::EPSILON);
    /// ```
    #[inline]
    pub fn rata_temporis(
        &self,
        pension_age: PensionAge,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<f64, RataTemporisError> {
        let m: u32 = self.actual_service(accuracy, rounding)?;
        let n: u32 = self.possible_service(pension_age, accuracy, rounding)?;

        if n == 0 {
            // No service possible
            Ok(0.0)
        } else {
            Ok(m as f64 / n as f64)
        }
    }

    /// Returns the [`RataTemporis`] as actual service (m) divided by possible service (n) based on the legal [`PensionAge`].
    ///
    /// The [`PensionAge`] is automatically calculated based on the birthyear using [`PensionAge::from_birthyear`].
    ///
    /// # Errors
    ///
    /// See [`RataTemporis::rata_temporis`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Date, RataTemporis, Accuracy, PensionAge, Rounding};
    /// let birth_date: Date = Date::new_num(2000, 1, 1).unwrap();
    /// let entry_date: Date = Date::new_num(2020, 1, 1).unwrap();
    /// let exit_date: Date = Date::new_num(2025, 12, 31).unwrap();
    /// let rata_temporis: RataTemporis = RataTemporis::new(birth_date, entry_date, exit_date).unwrap();
    ///
    /// // MonthExact
    /// let rata: f64 = rata_temporis.rata_temporis_birthyear(Accuracy::MonthExact, Rounding::Floor).unwrap();
    /// assert!((rata - 71. / 564.).abs() < f64::EPSILON);
    ///
    /// // DayExact
    /// let rata: f64 = rata_temporis.rata_temporis_birthyear(Accuracy::DayExact, Rounding::Floor).unwrap();
    /// assert!((rata - 2_191. / 17_167.).abs() < f64::EPSILON);
    ///
    /// // YearExact
    /// let rata: f64 = rata_temporis.rata_temporis_birthyear(Accuracy::YearExact, Rounding::Floor).unwrap();
    /// assert!((rata - 5. / 47.).abs() < f64::EPSILON);
    /// ```
    #[inline]
    pub fn rata_temporis_birthyear(
        &self,
        accuracy: Accuracy,
        rounding: Rounding,
    ) -> Result<f64, RataTemporisError> {
        let pension_age: PensionAge = PensionAge::from_birthyear(self.birth_date.year());

        self.rata_temporis(pension_age, accuracy, rounding)
    }
}

/// An enum for handling any errors involved in the calculation of [`RataTemporis`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RataTemporisError {
    /// Two [`Date`]s are not ordered as expected.
    WrongOrder { first_date: Date, second_date: Date },

    /// The addition of the [`PensionYears`] triggered a [`ChronoError`].
    YearError { pension_years: i32 },

    /// The addition of the [`PensionMonths`] triggered a [`ChronoError`].
    MonthError { pension_months: i32 },

    /// A negative difference was calculated.
    ///
    /// As the [`Date`]s of birth, entry, exit and pension are already checked for order, this should never happen.
    NegativeDifference,
}

impl RataTemporisError {
    /// Checks if two [`Date`]s are correctly ordered.
    ///
    /// `first_date` must be before `second_date`.
    #[inline]
    fn check_order(first_date: &Date, second_date: &Date) -> Result<(), RataTemporisError> {
        if first_date <= second_date {
            Ok(())
        } else {
            Err(Self::WrongOrder {
                first_date: *first_date,
                second_date: *second_date,
            })
        }
    }
}

impl Display for RataTemporisError {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RataTemporisError::WrongOrder {
                first_date,
                second_date,
            } => write!(
                format,
                "The first date {} was after the second date {}",
                first_date, second_date
            ),
            RataTemporisError::YearError { pension_years } => write!(
                format,
                "Addition of pension years ({}) unsuccessful",
                pension_years
            ),
            RataTemporisError::MonthError { pension_months } => write!(
                format,
                "Addition of pension months ({}) unsuccessful",
                pension_months
            ),
            RataTemporisError::NegativeDifference => {
                write!(format, "A negative difference was encountered")
            }
        }
    }
}

impl Error for RataTemporisError {}
