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


use super::Measurement;
use std::fmt;
use std::error::Error;
use std::convert::From;
use std::str::FromStr;

use super::ParseMeasurementError;

/// Internal representation of measurement list
///
/// # Example
///
/// ```
/// use orion::core::MeasurementsList;
///
/// let meas_list = MeasurementsList::new();
///
/// ```
#[derive(Debug)]
pub struct MeasurementsList {
    list : Vec<Measurement>,
}

impl MeasurementsList {

    /// Construct a new `MeasurementsList`
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::MeasurementsList;
    ///
    /// let meas_list = MeasurementsList::new();
    ///
    /// ```
    pub fn new() -> MeasurementsList {
        MeasurementsList {
            list: Vec::new(),
        }
    }
}

impl FromStr for MeasurementsList {

    type Err = ParseMeasurementsListError;


    /// Parse `MeasurementsList` from string
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::MeasurementsList;
    /// use orion::core::ParseMeasurementsListError;
    /// use std::str::FromStr;
    ///
    /// let ml = MeasurementsList::from_str("3.0[V] -5[A]").unwrap();
    /// let ml = MeasurementsList::from_str("-4.1[A] 3[W] 2.34[V]").unwrap();
    /// ```
    /// # Failure
    ///
    /// This function fail with:
    ///
    /// - ParseMeasurementsListError::InvalidFormat if string don't use a list
    ///   off measurement using this form : value[unit] and separred by one
    ///   comma.
    /// - ParseMeasurementError::InvalidMeasurement(ParseMeasurementError) if
    ///   one measurement is unparsable.
    ///
    /// ```
    /// use orion::core::MeasurementsList;
    /// use orion::core::ParseMeasurementsListError;
    /// use std::str::FromStr;
    ///
    /// assert!(
    ///     MeasurementsList::from_str("4x4[Car] 3[V]").is_err()
    /// );
    ///
    /// assert!(
    ///     MeasurementsList::from_str("3.0V 4A").is_err()
    /// );
    /// ```
    fn from_str(s: &str) -> Result<MeasurementsList, ParseMeasurementsListError>{
        let re = regex!( r"[^\[\] ]*\[[^\] ]*\]");

        if ! re.is_match(s) {
            return Err(ParseMeasurementsListError::InvalidFormat);
        }

        let mut list : Vec<Measurement> = Vec::new();

        for item in s.split(' ') {
            let measurement = try!( Measurement::from_str(item) );

            list.push(measurement);
        }

        Ok( MeasurementsList {
                list: list,
            }
        )
    }

}

impl fmt::Display for MeasurementsList {


    /// Format `MesurementsList` to `str`
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::MeasurementsList;
    /// use std::str::FromStr;
    /// use std::fmt::Display;
    ///
    /// let ml = MeasurementsList::from_str("3[V] -3.4[A]").unwrap();
    /// println!("{}", ml);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        let mut first_flag = true;
        for meas in self.list.iter() {
            if first_flag {
                first_flag = false;
            } else {
                try!( write!(f, " ") );
            }

            try!( write!(f,"{}", meas) );
        }

        Ok( () )

    }
}


#[derive(Debug)]
pub enum ParseMeasurementsListError {
    InvalidFormat,
    InvalidMeasurement(ParseMeasurementError),
}

impl fmt::Display for ParseMeasurementsListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        self.description().fmt(f)
    }
}

impl Error for ParseMeasurementsListError {
    fn description(&self) -> &str {
        match *self {
            ParseMeasurementsListError::InvalidMeasurement(_)  => "Invalid measurement",
            ParseMeasurementsListError::InvalidFormat => "Invalid format",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseMeasurementsListError::InvalidMeasurement(ref err) => Some(err as &Error),
            ParseMeasurementsListError::InvalidFormat         => None,
        }
    }
}

impl From<ParseMeasurementError> for ParseMeasurementsListError {
    fn from(err: ParseMeasurementError) -> ParseMeasurementsListError {
        ParseMeasurementsListError::InvalidMeasurement(err)
    }
}


#[test]
fn test_measurements_list_from_str() {
    assert!( MeasurementsList::from_str("3[V]").is_ok() );
    assert!( MeasurementsList::from_str("4.67[kg] -465[A]").is_ok() );
    assert!( MeasurementsList::from_str("-117[A] 33.334[W] -2[s]").is_ok() );

    let err_fmt = match MeasurementsList::from_str("") {
        Err(x)  =>  x,
        Ok(_)   =>  unreachable!(),
    };
    assert_eq!(err_fmt.description(), "Invalid format");

    let err_fmt = match MeasurementsList::from_str("value[V]") {
        Err(x)  =>  x,
        Ok(_)   =>  unreachable!(),
    };
    assert_eq!(err_fmt.description(), "Invalid measurement");
}

#[test]
fn test_measurements_list_to_string() {
    // to_string use fmt method
    assert_eq!( "3[V]", MeasurementsList::from_str("3[V]")
                                         .unwrap()
                                         .to_string()
    );
    assert_eq!("1.1[A] -3[V]", MeasurementsList::from_str("1.1[A] -3[V]")
                                                .unwrap()
                                                .to_string()
    );
}
