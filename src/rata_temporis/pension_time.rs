//! This module contains the implementation of the [`PensionAge`] struct and its predecessors.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::Year;

/// Handles the amount of months between `birth_date` and `pension_date`.
///
/// This must be a value between 0 and 11.
/// For more than 12 months use the [`PensionYears`] struct.
///
/// The [`Default`] is `0` months.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PensionMonths {
    pension_months: u8,
}

impl PensionMonths {
    /// Creates a new [`PensionMonths`] instance.
    ///
    /// # Errors
    ///
    /// * [`PensionAgeError::MonthError`] - The month is larger than 11.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionMonths, PensionAgeError};
    /// // Valid
    /// let pension_months: PensionMonths = PensionMonths::new(3).unwrap();
    /// assert_eq!(pension_months.value(), 3);
    ///
    /// // MonthError
    /// let month_error: PensionAgeError = PensionMonths::new(12).err().unwrap();
    /// assert_eq!(month_error, PensionAgeError::MonthError { pension_months: 12 });
    /// ```
    #[inline]
    pub const fn new(pension_months: u8) -> Result<Self, PensionAgeError> {
        if pension_months < 12 {
            Ok(Self { pension_months })
        } else {
            Err(PensionAgeError::MonthError { pension_months })
        }
    }

    /// Returns the [`PensionMonths`] based on SGB VI § 235.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionMonths, Year};
    /// // After transition
    /// let pension_months: PensionMonths = PensionMonths::from_birthyear(Year::try_from(2000).unwrap());
    /// assert_eq!(pension_months.value(), 0);
    ///
    /// // Before transition
    /// let pension_months: PensionMonths = PensionMonths::from_birthyear(Year::try_from(1946).unwrap());
    /// assert_eq!(pension_months.value(), 0);
    ///
    /// // During transition
    /// let pension_months: PensionMonths = PensionMonths::from_birthyear(Year::try_from(1952).unwrap());
    /// assert_eq!(pension_months.value(), 6);
    /// ```
    #[must_use]
    pub const fn from_birthyear(birthyear: Year) -> PensionMonths {
        let birthyear: i32 = birthyear.value();

        let pension_months: u8 = match birthyear {
            ..=1946 => 0,
            1947 => 1,
            1948 => 2,
            1949 => 3,
            1950 => 4,
            1951 => 5,
            1952 => 6,
            1953 => 7,
            1954 => 8,
            1955 => 9,
            1956 => 10,
            1957 => 11,
            1958 => 0,
            1959 => 2,
            1960 => 4,
            1961 => 6,
            1962 => 8,
            1963 => 10,
            1964.. => 0,
        };

        PensionMonths { pension_months }
    }

    /// Returns the amount of `pension_months`.
    #[must_use]
    #[inline]
    pub const fn value(&self) -> u8 {
        self.pension_months
    }
}

impl Default for PensionMonths {
    #[inline]
    fn default() -> Self {
        PensionMonths { pension_months: 0 }
    }
}

impl Display for PensionMonths {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        let months = self.value();
        write!(
            format,
            "{} month{}",
            months,
            if months == 1 { "" } else { "s" }
        )
    }
}

impl From<PensionMonths> for i32 {
    #[inline]
    fn from(pension_months: PensionMonths) -> Self {
        pension_months.value() as i32
    }
}

impl From<PensionMonths> for u8 {
    #[inline]
    fn from(pension_months: PensionMonths) -> Self {
        pension_months.value()
    }
}

impl From<PensionMonths> for usize {
    #[inline]
    fn from(pension_months: PensionMonths) -> Self {
        pension_months.value() as usize
    }
}

impl TryFrom<u8> for PensionMonths {
    type Error = PensionAgeError;

    #[inline]
    fn try_from(pension_months: u8) -> Result<Self, Self::Error> {
        PensionMonths::new(pension_months)
    }
}

/// Handles the amount of years between `birth_date` and `pension_date`.
///
/// This must be a value between [`PensionYears::MIN`] and [`PensionYears::MAX`].
///
/// The [`Default`] is `65` years.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PensionYears {
    pension_years: u8,
}

impl PensionYears {
    /// The smallest valid [`PensionYears`].
    pub const MIN: PensionYears = PensionYears { pension_years: 55 };

    /// The largest valid [`PensionYears`].
    pub const MAX: PensionYears = PensionYears { pension_years: 75 };

    /// Creates a new [`PensionYears`] instance.
    ///
    /// # Errors
    ///
    /// * [`PensionAgeError::YearError`] - The year is smaller than [`PensionYears::MIN`] or larger than [`PensionYears::MAX`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionYears, PensionAgeError};
    /// // Valid
    /// let pension_years: PensionYears = PensionYears::new(65).unwrap();
    /// assert_eq!(pension_years.value(), 65);
    ///
    /// // YearError
    /// let year_error: PensionAgeError = PensionYears::new(50).err().unwrap();
    /// assert_eq!(year_error, PensionAgeError::YearError { pension_years: 50 });
    /// // YearError
    /// let year_error: PensionAgeError = PensionYears::new(90).err().unwrap();
    /// assert_eq!(year_error, PensionAgeError::YearError { pension_years: 90 });
    /// ```
    #[inline]
    pub const fn new(pension_years: u8) -> Result<Self, PensionAgeError> {
        if pension_years >= Self::MIN.pension_years && pension_years <= Self::MAX.pension_years {
            Ok(Self { pension_years })
        } else {
            Err(PensionAgeError::YearError { pension_years })
        }
    }

    /// Returns the [`PensionYears`] based on SGB VI § 235.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionYears, Year};
    /// // After transition
    /// let pension_years: PensionYears = PensionYears::from_birthyear(Year::try_from(2000).unwrap());
    /// assert_eq!(pension_years.value(), 67);
    ///
    /// // Before transition
    /// let pension_years: PensionYears = PensionYears::from_birthyear(Year::try_from(1946).unwrap());
    /// assert_eq!(pension_years.value(), 65);
    ///
    /// // During transition
    /// let pension_years: PensionYears = PensionYears::from_birthyear(Year::try_from(1959).unwrap());
    /// assert_eq!(pension_years.value(), 66);
    /// ```
    #[must_use]
    pub const fn from_birthyear(birthyear: Year) -> PensionYears {
        let birthyear: i32 = birthyear.value();

        let pension_years: u8 = match birthyear {
            ..=1946 => 65,
            1947..=1957 => 65,
            1958..=1963 => 66,
            1964.. => 67,
        };

        PensionYears { pension_years }
    }

    /// Returns the amount of `pension_years`.
    #[must_use]
    #[inline]
    pub const fn value(&self) -> u8 {
        self.pension_years
    }
}

impl Default for PensionYears {
    #[inline]
    fn default() -> Self {
        PensionYears { pension_years: 65 }
    }
}

impl Display for PensionYears {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        let years = self.value();
        write!(
            format,
            "{} year{}",
            years,
            if years == 1 { "" } else { "s" }
        )
    }
}

impl From<PensionYears> for i32 {
    #[inline]
    fn from(pension_years: PensionYears) -> Self {
        pension_years.value() as i32
    }
}

impl From<PensionYears> for u8 {
    #[inline]
    fn from(pension_years: PensionYears) -> Self {
        pension_years.value()
    }
}

impl From<PensionYears> for usize {
    #[inline]
    fn from(pension_years: PensionYears) -> Self {
        pension_years.value() as usize
    }
}

impl TryFrom<u8> for PensionYears {
    type Error = PensionAgeError;

    #[inline]
    fn try_from(pension_years: u8) -> Result<Self, Self::Error> {
        PensionYears::new(pension_years)
    }
}

/// Handles the amount of years and months between `birth_date` and `pension_date`.
///
/// This is based on [`PensionMonths`] and [`PensionYears`].
///
/// The [`Default`] is `65` years and `0` months.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PensionAge {
    pension_years: PensionYears,
    pension_months: PensionMonths,
}

impl PensionAge {
    /// Creates a new [`PensionAge`] instance by combining [`PensionMonths`] and [`PensionYears`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionAge, PensionMonths, PensionYears};
    /// let pension_months: PensionMonths = PensionMonths::new(2).unwrap();
    /// let pension_years: PensionYears = PensionYears::new(65).unwrap();
    /// let pension_age: PensionAge = PensionAge::new(pension_years, pension_months);
    /// assert_eq!(pension_age.pension_years().value(), 65);
    /// assert_eq!(pension_age.pension_months().value(), 2);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(pension_years: PensionYears, pension_months: PensionMonths) -> Self {
        Self {
            pension_years,
            pension_months,
        }
    }

    /// Creates a new [`PensionAge`] instance by creating new [`PensionMonths`] and [`PensionYears`].
    ///
    /// # Errors
    ///
    /// See [`PensionYears::new`] and [`PensionMonths::new`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionAge, PensionAgeError};
    /// // Valid
    /// let pension_age: PensionAge = PensionAge::new_num(65, 2).unwrap();
    /// assert_eq!(pension_age.pension_years().value(), 65);
    /// assert_eq!(pension_age.pension_months().value(), 2);
    ///
    /// // YearError
    /// let year_error: PensionAgeError = PensionAge::new_num(90, 3).err().unwrap();
    /// assert_eq!(year_error, PensionAgeError::YearError { pension_years: 90 });
    ///
    /// // MonthError
    /// let month_error: PensionAgeError = PensionAge::new_num(65, 13).err().unwrap();
    /// assert_eq!(month_error, PensionAgeError::MonthError { pension_months: 13 });
    ///
    /// // Both (YearError is triggered first)
    /// let both: PensionAgeError = PensionAge::new_num(90, 13).err().unwrap();
    /// assert_eq!(both, PensionAgeError::YearError { pension_years: 90 });
    /// ```
    #[inline]
    pub fn new_num(pension_years: u8, pension_months: u8) -> Result<Self, PensionAgeError> {
        let pension_years: PensionYears = PensionYears::new(pension_years)?;
        let pension_months: PensionMonths = PensionMonths::new(pension_months)?;

        Ok(Self {
            pension_years,
            pension_months,
        })
    }

    /// Creates a new [`PensionAge`] instance at exactly 65 years and 0 months.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::PensionAge;
    /// let just_65: PensionAge = PensionAge::just_65();
    /// assert_eq!(just_65.pension_years().value(), 65);
    /// assert_eq!(just_65.pension_months().value(), 0);
    /// ```
    #[must_use]
    #[inline]
    pub const fn just_65() -> PensionAge {
        Self::just_age(65)
    }

    /// Creates a new [`PensionAge`] instance at exactly 60 years and 0 months.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::PensionAge;
    /// let just_60: PensionAge = PensionAge::just_60();
    /// assert_eq!(just_60.pension_years().value(), 60);
    /// assert_eq!(just_60.pension_months().value(), 0);
    /// ```
    #[must_use]
    #[inline]
    pub const fn just_60() -> PensionAge {
        Self::just_age(60)
    }

    /// Creates a new [`PensionAge`] instance at exactly 63 years and 0 months.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::PensionAge;
    /// let just_63: PensionAge = PensionAge::just_63();
    /// assert_eq!(just_63.pension_years().value(), 63);
    /// assert_eq!(just_63.pension_months().value(), 0);
    /// ```
    #[must_use]
    #[inline]
    pub const fn just_63() -> PensionAge {
        Self::just_age(63)
    }

    /// Returns the [`PensionAge`] based on SGB VI § 235.
    ///
    /// See [`PensionMonths::from_birthyear`] and [`PensionYears::from_birthyear`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionAge, Year};
    /// // After transition
    /// let birth_year: Year = Year::new(2000).unwrap();
    /// let pension_age: PensionAge = PensionAge::from_birthyear(birth_year);
    /// assert_eq!(pension_age.pension_years().value(), 67);
    /// assert_eq!(pension_age.pension_months().value(), 0);
    ///
    /// // Before transition
    /// let birth_year: Year = Year::new(1946).unwrap();
    /// let pension_age: PensionAge = PensionAge::from_birthyear(birth_year);
    /// assert_eq!(pension_age.pension_years().value(), 65);
    /// assert_eq!(pension_age.pension_months().value(), 0);
    ///
    /// // During transition
    /// let birth_year: Year = Year::new(1959).unwrap();
    /// let pension_age: PensionAge = PensionAge::from_birthyear(birth_year);
    /// assert_eq!(pension_age.pension_years().value(), 66);
    /// assert_eq!(pension_age.pension_months().value(), 2);
    /// ```
    #[must_use]
    pub const fn from_birthyear(birthyear: Year) -> PensionAge {
        let pension_years: PensionYears = PensionYears::from_birthyear(birthyear);
        let pension_months: PensionMonths = PensionMonths::from_birthyear(birthyear);

        PensionAge {
            pension_years,
            pension_months,
        }
    }

    /// Returns the [`PensionYears`].
    #[must_use]
    #[inline]
    pub const fn pension_years(&self) -> PensionYears {
        self.pension_years
    }

    /// Returns the [`PensionMonths`].
    #[must_use]
    #[inline]
    pub const fn pension_months(&self) -> PensionMonths {
        self.pension_months
    }

    /// Calculates the total amount of months based on a given [`PensionAge`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{PensionAge, Year};
    /// let pension_age: PensionAge = PensionAge::just_65();
    /// assert_eq!(pension_age.total_months(), 780);
    ///
    /// let year: Year = Year::try_from(1959).unwrap();
    /// let pension_age: PensionAge = PensionAge::from_birthyear(year);
    /// assert_eq!(pension_age.total_months(), 794)
    /// ```
    #[must_use]
    #[inline]
    pub const fn total_months(&self) -> u32 {
        (self.pension_years.value() as u32 * 12) + self.pension_months.value() as u32
    }

    /// Creates a new [`PensionAge`] instance at exactly * years and 0 months.
    ///
    ///
    /// # Safety
    ///
    /// This does not involve any checks against [`PensionYears::MIN`] or [`PensionYears::MAX`].
    /// It directly constructs the [`PensionYears`] and [`PensionMonths`].
    /// It is the callers responsibility to ensure the provided `age` is valid!
    #[inline]
    const fn just_age(age: u8) -> PensionAge {
        // Age must be valid
        let pension_years: PensionYears = PensionYears { pension_years: age };
        let pension_months: PensionMonths = PensionMonths { pension_months: 0 };

        Self {
            pension_years,
            pension_months,
        }
    }
}

impl Default for PensionAge {
    #[inline]
    fn default() -> Self {
        Self {
            pension_years: PensionYears::default(),
            pension_months: PensionMonths::default(),
        }
    }
}

impl Display for PensionAge {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{} and {}", self.pension_years, self.pension_months)
    }
}

/// An enum for handling any errors involved in the creation of [`PensionMonths`] and [`PensionYears`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PensionAgeError {
    /// The [`PensionMonths`] is greater than `11`.
    MonthError { pension_months: u8 },

    /// The [`PensionYears`] is smaller than [`PensionYears::MIN`] or larger than [`PensionYears::MAX`].
    YearError { pension_years: u8 },
}

impl Display for PensionAgeError {
    fn fmt(&self, format: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            PensionAgeError::MonthError { pension_months } => write!(
                format,
                "Pension months ({}) must be inside the interval [0, 11]",
                pension_months
            ),
            PensionAgeError::YearError { pension_years } => write!(
                format,
                "Pension years ({}) must be inside the interval [{}, {}]",
                pension_years,
                PensionYears::MIN.pension_years,
                PensionYears::MAX.pension_years
            ),
        }
    }
}

impl Error for PensionAgeError {}
