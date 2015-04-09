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


use super::Unit;
use std::fmt;
use std::error::Error;
use std::convert::From;
use std::str::FromStr;

use super::ParseUnitError;
use std::num::ParseFloatError;

/// Internal representation of measurement
///
/// # Example
///
/// ```
/// use orion::core::Measurement;
/// use orion::core::Unit;
///
/// let meas = Measurement::new(4.0, Unit::Ampere);
///
/// ```
#[derive(Debug)]
pub struct Measurement {
    value: f32,
    unit:  Unit,
}

impl Measurement {

    /// Construct a new `measurement` with given `value` and `unit`
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::Measurement;
    /// use orion::core::Unit;
    ///
    /// let meas = Measurement::new( 3.0, Unit::Volt);
    ///
    /// ```
    pub fn new(value : f32, unit : Unit) -> Measurement {
        Measurement {
            value: value,
            unit: unit,
        }
    }
}


impl FromStr for Measurement {

    type Err = ParseMeasurementError;


    /// Parse measurement from string
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::{Measurement,ParseMeasurementError};
    /// use std::str::FromStr;
    ///
    /// let meas = Measurement::from_str("3.0[V]").unwrap();
    /// let meas = Measurement::from_str("-4.1[A]").unwrap();
    /// ```
    /// # Failure
    ///
    /// This function fail with:
    ///
    /// - ParseMeasurementError::InvalidFormat if string don't hase this
    ///   form : value[unit]
    /// - ParseMeasurementError::InvalidValue(ParseFloatError) if value don't
    ///   represent a float number. Valid value example : `3.0`, `-4.15`,
    ///   `-156.75865`
    /// - ParseMeasurementError::InvalidUnit(ParseUnitError) if unit don't
    ///   represent a `Unit`. Valid unit example: `A`, `V`, `Ω`
    ///
    /// ```
    /// use orion::core::{Measurement,ParseMeasurementError};
    /// use std::str::FromStr;
    /// assert!(
    ///     Measurement::from_str("4x4[Car]").is_err()
    /// );
    fn from_str(s: &str) -> Result<Measurement, ParseMeasurementError>{

        let re = regex!(r"^([^\[\]]*)\[([^\[\]]*)\]$");

        let data = match re.captures(s) {
            Some(x) => x,
            None    => return Err(ParseMeasurementError::InvalidFormat),
        };

        let extracted_value = match data.at(1) {
            Some(x) => x,
            None    => unreachable!(),
        };
        trace!("Measurement.from_str : Value -> {}", extracted_value);

        let extracted_unit = match data.at(2) {
            Some(x) => x,
            None    => unreachable!(),
        };
        trace!("Measurement.from_str : Unit => {}", extracted_unit);


        let value = try!( f32::from_str(extracted_value) );
        let unit = try!( Unit::from_str(extracted_unit) );

        Ok (
            Measurement {
                value : value,
                unit  : unit,
            }
        )


    }

}


impl fmt::Display for Measurement {


    /// Format `Meaurement` to `str`
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::Measurement;
    /// use orion::core::Unit;
    /// use std::fmt::Display;
    ///
    /// let meas = Measurement::new(3.0, Unit::Volt);
    /// println!("{}", meas);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}[{}]", self.value, self.unit)
    }
}

#[derive(Debug)]
pub enum ParseMeasurementError {
    InvalidValue(ParseFloatError),
    InvalidUnit(ParseUnitError),
    InvalidFormat,
}

impl fmt::Display for ParseMeasurementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        self.description().fmt(f)
    }
}

impl Error for ParseMeasurementError {
    fn description(&self) -> &str {
        match *self {
            ParseMeasurementError::InvalidValue(_)  => "Invalid value",
            ParseMeasurementError::InvalidUnit(_)   => "Invalid unit",
            ParseMeasurementError::InvalidFormat => "Invalid format",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseMeasurementError::InvalidValue(ref err) => Some(err as &Error),
            ParseMeasurementError::InvalidUnit(ref err)  => Some(err as &Error),
            ParseMeasurementError::InvalidFormat         => None,
        }
    }
}

impl From<ParseUnitError> for ParseMeasurementError {
    fn from(err: ParseUnitError) -> ParseMeasurementError {
        ParseMeasurementError::InvalidUnit(err)
    }
}

impl From<ParseFloatError> for ParseMeasurementError {
    fn from(err: ParseFloatError) -> ParseMeasurementError {
        ParseMeasurementError::InvalidValue(err)
    }
}


#[test]
fn measurement_from_str() {
    assert!( Measurement::from_str("3[V]").is_ok() );
    assert!( Measurement::from_str("4.67[kg]").is_ok() );
    assert!( Measurement::from_str("-117[A]").is_ok() );
    assert!( Measurement::from_str("-185753.457568657[W]").is_ok() );
    assert!( Measurement::from_str("4645765.454567554[Ω]").is_ok() );

    let err_fmt = match Measurement::from_str("") {
        Err(x)  =>  x,
        Ok(_)   =>  unreachable!(),
    };
    assert_eq!(err_fmt.description(), "Invalid format");

    let err_fmt = match Measurement::from_str("value[V]") {
        Err(x)  =>  x,
        Ok(_)   =>  unreachable!(),
    };
    assert_eq!(err_fmt.description(), "Invalid value");

    let err_fmt = match Measurement::from_str("4.4[cars]") {
        Err(x)  =>  x,
        Ok(_)   =>  unreachable!(),
    };
    assert_eq!(err_fmt.description(), "Invalid unit");
}

#[test]
fn measurement_to_string() {
    // to_string use fmt method
    assert_eq!( "3[V]", Measurement::new(3.0, Unit::Volt).to_string() );
    assert_eq!("1.1234[A]", Measurement::new(1.1234, Unit::Ampere).to_string() );
    assert_eq!("-124[kg]", Measurement::new(-124.0, Unit::Kilogram).to_string() );
    assert_eq!("-12.2[s]", Measurement::new(-12.2, Unit::Second).to_string() );
}
