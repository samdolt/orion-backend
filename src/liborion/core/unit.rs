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
    /// let v = Unit::from_str("V").unwrap();
    /// ```
    /// # Failure
    ///
    /// This function fail with `Err(ParseUnitError::Invalid)` if :
    ///
    /// - Unit is not one of `V`, `A`, `Ω`, `W`, `K`, `s` or `kg`
    ///
    /// ```
    /// use orion::core::{Unit,ParseUnitError};
    /// use std::str::FromStr;
    /// assert!(
    ///     Unit::from_str("wrong_unit").is_err()
    /// );
    fn from_str(s: &str) -> Result<Unit, ParseUnitError>{

        let re = regex!(r"^(V|A|Ω|W|K|s|kg)$");

        let data = match re.captures(s) {
            Some(x) => x,
            None    => return Err(ParseUnitError::Invalid),
        };

        let extracted_content = match data.at(1) {
            Some(x) => x,
            None    => unreachable!(),
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

impl fmt::Display for Unit {


    /// Format `Unit` to `str`
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::Unit;
    /// use std::fmt::Display;
    ///
    /// let unit = Unit::Volt;
    /// println!("3 {}", unit);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Unit::Volt     => write!(f, "{}", "V"),
            Unit::Ohm      => write!(f, "{}", "Ω"),
            Unit::Ampere   => write!(f, "{}", "A"),
            Unit::Watt     => write!(f, "{}", "W"),
            Unit::Kelvin   => write!(f, "{}", "K"),
            Unit::Second   => write!(f, "{}", "s"),
            Unit::Kilogram => write!(f, "{}", "kg"),
        }
    }
}

#[test]
fn test_unit_from_str() {

    assert!( Unit::from_str("V").is_ok() );
    assert!( Unit::from_str("Ω").is_ok() );
    assert!( Unit::from_str("A").is_ok() );
    assert!( Unit::from_str("W").is_ok() );
    assert!( Unit::from_str("K").is_ok() );
    assert!( Unit::from_str("s").is_ok() );
    assert!( Unit::from_str("kg").is_ok() );

    assert!( Unit::from_str("[V]").is_err() );
    assert!( Unit::from_str("super_unit").is_err() );
    assert!( Unit::from_str("cars").is_err() );

}

#[test]
fn test_unit_to_string() {
    // to_string use fmt method

    assert_eq!( Unit::Volt.to_string()     , "V" );
    assert_eq!( Unit::Ohm.to_string()      , "Ω" );
    assert_eq!( Unit::Ampere.to_string()   , "A" );
    assert_eq!( Unit::Watt.to_string()     , "W" );
    assert_eq!( Unit::Kelvin.to_string()   , "K" );
    assert_eq!( Unit::Second.to_string()   , "s" );
    assert_eq!( Unit::Kilogram.to_string() , "kg" );
}

