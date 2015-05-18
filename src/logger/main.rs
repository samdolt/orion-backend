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


extern crate docopt;
extern crate rustc_serialize;
extern crate nanomsg;
#[macro_use] extern crate log;
#[macro_use] extern crate patch;


#[macro_use] extern crate regex;


extern crate env_logger;
extern crate chrono;

extern crate orion;

use docopt::Docopt;

pub mod validator;
pub mod messages;
use messages::*;

pub mod add;
pub mod server;

pub static DATA_PATH: &'static str = "/tmp/data";

static USAGE: &'static str = "
Orion Backend

Usage:
    orion-logger [-v --debug] add <value> --now from <device>
    orion-logger [-v --debug] add <value> --timestamp=<timestamp> from <device>
    orion-logger [-v --debug] server (start | stop)
    orion-logger -h | --help
    orion-logger --version

Options:
    --now                     Use current time as timestamp
    --timestamp <timestamp>   Use an IETF RFC3339 timestamp
    -v, --verbose             Verbose output.
    -h, --help                Show help.
    --version                 Show version.
    --debug                   Very verbose output.

Commands:
    add                       Log a new set of data
    server                    Manage orion-logger server

See 'orion-logger help <command>' for more information on a specific command.

Notes:
    Combining --debug and --verbose enable `trace` level message

Report bugs to: <samuel@dolt.ch>
Orion home page: <http://orion.dolt.ch>
";

fn main() {
    let args : Args = Docopt::new(USAGE)
                            .and_then(|d|  d.decode())
                            .unwrap_or_else(|e| e.exit() );
    init_logger_with_args(&args);
    get_command(&args).run( args );
}


#[derive(Debug, RustcDecodable, Copy, Clone)]
enum Command {
    Add,
    Server,
    Default,
}

impl Command {
    fn run ( &self, args: Args ) {
        match *self {
            Command::Add => add::run( args ),
            Command::Server => server::run( args ),
            Command::Default => default_cmd_run( args ),
        }
    }
}

fn get_command(args: &Args) -> Command {

    if args.cmd_add {
        Command::Add
    } else if args.cmd_server {
        Command::Server
    } else {
        Command::Default
    }
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

fn default_cmd_run(args: Args) {

    if args.flag_version {
        println!("Orion-Logger (Orion-Backend) {}", env!("CARGO_PKG_VERSION"));
        println!("{}", COPYRIGHT);
        return;

    }
}

#[derive(RustcDecodable, Debug)]
pub struct Args {
    cmd_server: bool,
    cmd_add: bool,
    cmd_start: bool,
    cmd_stop: bool,
    arg_device: String,
    arg_value: String,
    flag_timestamp: String,
    flag_now: bool,
    flag_verbose: bool,
    flag_help: bool,
    flag_version: bool,
    flag_debug: bool,
}
