// Copyright 2015 © Samuel Dolt <samuel@dolt.ch>
//
// This file is part of orion_backend.
//
// Orion_backend is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Orion_backend is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with orion_backend.  If not, see <http://www.gnu.org/licenses/>.

use std::str::FromStr;
use std::error::Error;
use std::fmt;

use regex;

/// Internal representation of unit (SI)
///
/// # Example
///
/// ```
/// use orion::core::Unit;
///
/// let Unit = Unit::Volt;
/// ```
#[derive(Debug)]
pub enum Unit {
    Volt,
    Ohm,
    Ampere,
    Watt,
    Kelvin,
    Second,
    Kilogram,
}
#[derive(Debug)]
pub enum ParseUnitError {
    Invalid,
}

impl fmt::Display for ParseUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        self.description().fmt(f)
    }
}
impl Error for ParseUnitError {
    fn description(&self) -> &str {
        match *self {
            ParseUnitError::Invalid  => "Invalid format or unit",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl FromStr for Unit {

    type Err = ParseUnitError;


    /// Parse unit from string
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::{Unit,ParseUnitError};
    /// use std::str::FromStr;
    ///
    /// let v = Unit::from_str("[V]").unwrap();
    /// ```
    /// # Failure
    ///
    /// This function fail with `Err(ParseUnitError::Invalid)` if :
    ///
    /// - String don't use `"[unit]"` format
    /// - Unit is not one of `V`, `A`, `Ω`, `W`, `K`, `s` or `kg`
    ///
    /// ```
    /// use orion::core::{Unit,ParseUnitError};
    /// use std::str::FromStr;
    /// assert!(
    ///     Unit::from_str("[wrong_unit]").is_err()
    /// );
    fn from_str(s: &str) -> Result<Unit, ParseUnitError>{

        let re = regex!(r"[\[](V|A|Ω|W|K|s|kg)[\]]$");

        let data = match re.captures(s) {
            Some(x) => x,
            None    => return Err(ParseUnitError::Invalid),
        };

        let extracted_content = match data.at(1) {
            Some(x) => x,
            None    => return Err(ParseUnitError::Invalid),
        };

        match extracted_content {
            "V"  => Ok(Unit::Volt),
            "Ω"  => Ok(Unit::Ohm),
            "A"  => Ok(Unit::Ampere),
            "W"  => Ok(Unit::Watt),
            "K"  => Ok(Unit::Kelvin),
            "s"  => Ok(Unit::Second),
            "kg" => Ok(Unit::Kilogram),
            _    => Err(ParseUnitError::Invalid),
        }

    }

}

