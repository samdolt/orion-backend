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

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use std::str::FromStr;
use chrono::{UTC, DateTime, Datelike};

use super::Args;
use super::validator::OrionLoggerValidator;
use super::messages::*;
use super::DATA_PATH;

use orion::core::*;

pub fn run ( args: Args ) {

    if args.flag_timestamp != "" {
        trace!("Testing args.flag_timestamp");

        if args.flag_timestamp.is_rfc3339_timestamp() == false {
            println!("{}", INVALID_TIMESTAMP);
            return;
        }
    }

    let meas_list = match MeasurementsList::from_str( &args.arg_value ) {
        Ok(x)   => x,
        Err(_)  => { print!("{}", INVALID_VALUE);
                    return
                   }
    };

    let device = match Device::with_slug( &args.arg_device ) {
        Some(x) => x,
        None  => { 
                    print!("{}", INVALID_DEVICE); 
                    return
        },
    };

    let data = MeasurementPoint { 
        date : if args.flag_now { 
                    UTC::now() 
               } else { 
                    DateTime::parse_from_rfc3339(
                        &args.flag_timestamp
                    ).unwrap().with_timezone(&UTC)
              }, 
        data: meas_list,
        device: device,
    };

    add_value(data).unwrap();
}


#[derive(Debug)]
struct MeasurementPoint {
    date: DateTime<UTC>,
    data: MeasurementsList,
    device: Device,
}

/// Return a `io::Result<File>` for the given `MeasurementPoint`
///
/// # Examples
///
/// ```
/// let mp = MeasurementPoint {
///     date : UTC::now(),
///     data : "Some data",
///     device : Device::with_slug("port@node.driver"),
/// }
///
/// let file = open_file_for(&mp).unwrap();
/// ```
///
/// This examples open this file:
///
///     $(DATA_PATH)/$(DRIVER)/$(NODE)/$(YEAR)/$(MONTH)/$DAY/$(PORT).dat
///
/// This function create every missing parent directory and open the file
/// whith `create`, `write` and `append` flags
///
/// See [`OpenOptions` from `std::fs`](http://doc.rust-lang.org/std/fs/struct.OpenOptions.html)
///
/// # Failures
///
/// This function can fail if:
///     - Invalid permission is set on folder $(DATA_PATH)
///     - $(DATA_PATH) is read only
///     - Other system error with file handling
fn open_file_for(mp: &MeasurementPoint) -> io::Result<File> {
    let path = Path::new(DATA_PATH)
                   .join(mp.device.get_driver())
                   .join(mp.device.get_node())
                   .join(mp.device.get_port())
                   .join(format!("{}", mp.date.year()))
                   .join(format!("{}", mp.date.month()))
                   .join(format!("{}", mp.date.day()));

    debug!("Create all parent directory of {:?}", path.as_path()); 
    try!(fs::create_dir_all(path.as_path()));

    let filename = "data.txt";
    let file_path = path.join(filename);

    debug!("Open or create file {:?}", file_path.as_path());

    OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(file_path)
}

fn create_line_for(mp: &MeasurementPoint) -> String {
    let mut line = String::with_capacity(80);

    debug!("Create_line_for {:?}", mp);

    line.push_str( &mp.date.to_rfc3339() );
    line.push(' ');

    line.push_str( &mp.data.to_string() );
    line.push('\n');

    debug!("Line: {}", line);

    line
}



fn add_value(mp: MeasurementPoint) -> io::Result<()> {
    let mut file = try!( open_file_for(&mp) );

    let line = create_line_for(&mp);

    debug!("Append line '{}' to file", line);
    try!(file.write_all(line.as_bytes()));
    Ok(())
}
