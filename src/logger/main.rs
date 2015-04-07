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

#![feature(convert)]
#![feature(file_path)]
#![feature(plugin)]
#![plugin(regex_macros)]

extern crate docopt;
extern crate rustc_serialize;
#[macro_use] extern crate log;


#[macro_use] extern crate regex;


extern crate env_logger;
extern crate chrono;

use docopt::Docopt;
use std::fs;

use std::fs::File;

use std::path::Path;
use std::io;
use std::io::Write;
use std::fs::OpenOptions;

use chrono::{UTC,DateTime,Datelike};


mod validator;
use validator::OrionLoggerValidator;

mod messages;
use messages::*;

mod device;
use device::Device;

static DATA_PATH: &'static str = "/tmp/data";

static USAGE: &'static str = "
Orion Backend

Usage:
    orion-logger [-v --debug] --now from <device> add <value>
    orion-logger [-v --debug] --timestamp=<timestamp> from <device> add <value>
    orion-logger -h | --help
    orion-logger --version

Options:
    --now                     Use current time as timestamp
    --timestamp <timestamp>   Use an IETF RFC3339 timestamp
    -v, --verbose             Verbose output.
    -h, --help                Show help.
    --version                 Show version.
    --debug                   Very verbose output.
    

Notes:
    Combining --debug and --verbose enable `trace` level message

Report bugs to: <samuel@dolt.ch>
Orion home page: <http://orion.dolt.ch>
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_device: String,
    arg_value: String,
    flag_timestamp: String,
    flag_now: bool,
    flag_verbose: bool,
    flag_help: bool,
    flag_version: bool,
    flag_debug: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("orion_backend (Orion) {}", env!("CARGO_PKG_VERSION"));
        println!("{}", COPYRIGHT);
        return;

    }

    init_logger_with_args(&args);
    
    if args.flag_timestamp != "" {
        trace!("Testing args.flag_timestamp");
        
        if args.flag_timestamp.is_RFC3339_timestamp() == false {
            println!("{}", INVALID_TIMESTAMP);
            return;
        }
    }

    if args.arg_value.is_measurements_list() == false {
        println!("{}", INVALID_VALUE);
        return;
    }
    
    let device = match Device::with_slug(args.arg_device.as_str()) {
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
                        args.flag_timestamp.as_str()
                    ).unwrap().with_timezone(&UTC)
              }, 
        data: args.arg_value,
        device: device,
    };
    
    add_value(data).unwrap();

}

fn init_logger_with_args( args: &Args ) {
    let key = "RUST_LOG";
    
    if args.flag_verbose && args.flag_debug {
        std::env::set_var(key, "TRACE");
    } else if args.flag_verbose {
        std::env::set_var(key, "INFO");
    } else if args.flag_debug {
        std::env::set_var(key, "DEBUG");
    } else {
        std::env::set_var(key, "WARN");
    }
    env_logger::init().unwrap();
}


#[derive(Debug)]
struct Measurement {
    value: String,
    value_modifier: String,
    
    unit: String,
    unit_modifier: String,
}

#[derive(Debug)]
struct MeasurementPoint {
    date: DateTime<UTC>,
    data: String,
    device: Device,
}


fn open_file_for(mp: &MeasurementPoint) -> io::Result<File> {
    let path = Path::new(DATA_PATH)
                   .join(mp.device.get_driver())
                   .join(mp.device.get_node())
                   .join(format!("{}", mp.date.year()))
                   .join(format!("{}", mp.date.month()))
                   .join(format!("{}", mp.date.day()));
    
    debug!("Create all parent directory of {:?}", path.as_path()); 
    try!(fs::create_dir_all(path.as_path()));
    
    let filename = format!("{}.dat", mp.device.get_port());
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
    
    line.push_str(mp.date.to_rfc3339().as_str());
    line.push(' ');
    
    line.push_str(mp.data.as_str());
    line.push('\n');
    
    debug!("Line: {}", line);
    
    line
}



fn add_value(mp: MeasurementPoint) -> io::Result<()> {
    let mut file = try!( open_file_for(&mp) );
    let line = create_line_for(&mp);
    
    debug!("Append line '{}' to {:?}", line, file.path());
    try!(file.write_all(line.as_bytes()));
    Ok(())
} 

