//! This module contains the implementation of the [`Age`] struct.

use std::fmt::{self, Display, Formatter};

use crate::ChronoError;

/// A representation of a persons [`Age`].
///
/// This is a wrapper around [`u8`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age {
    age: u8,
}

impl Age {
    /// Creates a new [`Age`] instance.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::AgeError`] - `age` < [`Age::MIN`] or `age` > [`Age::MAX`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Age, ChronoError};
    /// // Valid
    /// let age: Age = Age::new(29).unwrap();
    /// assert_eq!(age.value(), 29);
    ///
    /// // AgeError
    /// let age_error: ChronoError = Age::new(200).err().unwrap();
    /// assert_eq!(age_error, ChronoError::AgeError(200));
    /// ```
    #[inline]
    pub fn new(age: u8) -> Result<Age, ChronoError> {
        if age <= Self::MAX && age >= Self::MIN {
            Ok(Age { age })
        } else {
            Err(ChronoError::AgeError(age))
        }
    }

    /// Returns a new [`Age`] instance without any checks.
    ///
    /// # Panics
    ///
    /// The `age` is not between [`Age::MIN`] and [`Age::MAX`] both included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::Age;
    /// const AGE: Age = Age::new_const(20);
    /// ```
    pub const fn new_const(age: u8) -> Self {
        if age <= Self::MAX && age >= Self::MIN {
            Age { age }
        } else {
            panic!("Invalid age")
        }
    }

    /// Creates a new [`Age`] instance based on a string.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::AgeError`] - Something in [`Age::new`] went wrong.
    /// * [`ChronoError::ParseError`] - Could not parse `string` as [`u8`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Age, ChronoError};
    /// // Valid
    /// let age: Age = Age::from_string("29").unwrap();
    /// assert_eq!(age.value(), 29);
    ///
    /// // AgeError
    /// let age_error: ChronoError = Age::from_string("200").err().unwrap();
    /// assert_eq!(age_error, ChronoError::AgeError(200));
    ///
    /// // ParseError
    /// let parse_error: ChronoError = Age::from_string("Twenty").err().unwrap();
    /// assert_eq!(parse_error, ChronoError::ParseError(String::from("Twenty")));
    /// ```
    #[inline]
    pub fn from_string(string: &str) -> Result<Age, ChronoError> {
        let age: u8 = string
            .parse()
            .map_err(|_| ChronoError::ParseError(String::from(string)))?;

        Self::new(age)
    }

    /// Returns the value of the [`Age`] instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Age, ChronoError};
    /// let age: Age = Age::new(29).unwrap();
    /// assert_eq!(age.value(), 29);
    /// ```
    #[inline]
    pub const fn value(&self) -> u8 {
        self.age
    }

    /// Adds a number of years to a [`Age`] instance.
    ///
    /// To subtract use a negative sign.
    ///
    /// # Errors
    ///
    /// * [`ChronoError::AgeError`] - Something in [`Age::new`] went wrong.
    /// * [`ChronoError::OverflowError`] - The `years` argument was too large.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use date::{Age, ChronoError};
    /// // Valid
    /// let age: Age = Age::new(30).unwrap();
    /// let new_age: Age = age.add_years(20).unwrap();
    /// assert_eq!(new_age.value(), 50);
    ///
    /// // OverflowError
    /// let age: Age = Age::new(110).unwrap();
    /// let overflow_error: ChronoError = age.add_years(i32::MAX).err().unwrap();
    /// assert_eq!(overflow_error, ChronoError::OverflowError);
    ///
    /// let age: Age = Age::new(20).unwrap();
    /// let overflow_error: ChronoError = age.add_years(Age::MAX as i32).err().unwrap();
    /// assert_eq!(overflow_error, ChronoError::AgeError(20 + Age::MAX));
    /// ```
    #[inline]
    pub fn add_years(&self, years: i32) -> Result<Self, ChronoError> {
        let new_age: u8 = (self.age as i32)
            .checked_add(years)
            .ok_or(ChronoError::OverflowError)?
            .try_into()
            .map_err(|_| ChronoError::OverflowError)?;

        Self::new(new_age)
    }

    /// The smallest reasonable age a person should have.
    pub const MIN: u8 = 0_u8;

    /// The largest reasonable age a person should have.
    ///
    /// Oldest person ever was 122.5 years old
    pub const MAX: u8 = 115_u8;
}

impl Display for Age {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "{}", self.age)
    }
}

impl TryFrom<u8> for Age {
    type Error = ChronoError;

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        Self::new(number)
    }
}

impl TryFrom<usize> for Age {
    type Error = ChronoError;

    fn try_from(number: usize) -> Result<Self, Self::Error> {
        let as_u8: u8 = number.try_into().map_err(|_| ChronoError::OverflowError)?;
        Self::new(as_u8)
    }
}

impl TryFrom<i32> for Age {
    type Error = ChronoError;

    fn try_from(number: i32) -> Result<Self, Self::Error> {
        let as_u8: u8 = number.try_into().map_err(|_| ChronoError::OverflowError)?;
        Self::new(as_u8)
    }
}

impl From<Age> for u8 {
    fn from(age: Age) -> Self {
        age.value()
    }
}

impl From<Age> for usize {
    fn from(age: Age) -> Self {
        age.value() as usize
    }
}

impl From<Age> for i32 {
    fn from(age: Age) -> Self {
        age.value() as i32
    }
}
