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

use super::Args;
use super::DATA_PATH;

use orion::core::*;

use nanomsg::{Socket, Protocol};
use std::thread;
use std::io::{Read, Write};
use std::sync::mpsc::channel;

pub fn run ( args: Args ) {
    trace!("Logger server command");

    if args.cmd_start {
        start();
    } else if args.cmd_stop {
        stop();
    } else {
        panic!("Undefined task in Logger server");
    }
}

const CLIENT_DEVICE_URL: &'static str = "ipc:///tmp/reqrep_example_front.ipc";
const SERVER_DEVICE_URL: &'static str = "ipc:///tmp/reqrep_example_back.ipc";

pub fn start() {
    trace!("Logger server task 'start'");

    thread::spawn( move || {
        let mut front_socket = Socket::new_for_device(Protocol::Rep).unwrap();
        let mut front_endpoint = front_socket.bind(CLIENT_DEVICE_URL).unwrap();
        let mut back_socket = Socket::new_for_device(Protocol::Req).unwrap();
        let mut back_endpoint = back_socket.bind(SERVER_DEVICE_URL).unwrap();

        println!("Device is ready.");
        Socket::device(&front_socket, &back_socket);
        println!("Device is stopped.");

        front_endpoint.shutdown();
        back_endpoint.shutdown();
    });

    let mut socket = Socket::new(Protocol::Rep).unwrap();
    let mut endpoint = socket.connect(SERVER_DEVICE_URL).unwrap();
    let mut count = 1u32;

    let mut request = String::new();

    println!("Server is ready.");

    loop {

        match socket.read_to_string(&mut request) {
            Ok(_) => {
                let mut q_flag = false;
                println!("Recv '{}'.", request);

                let reply = if request == "LOGGER/1.0 STOP" {
                    q_flag = true;
                     "LOGGER/1.0 OK".to_string()
                } else {
                    format!("{} -> Reply #{}", request, count)
                };

                match socket.write_all(reply.as_bytes()) {
                    Ok(..) => println!("Sent '{}'.", reply),
                    Err(err) => {
                        println!("Server failed to send reply '{}'.", err);
                        break
                    }
                }
                request.clear();

                if q_flag {
                    break
                }

                thread::sleep_ms(400);
                count += 1;
            },
            Err(err) => {
                println!("Server failed to receive request '{}'.", err);
                break
            }
        }
    }

    endpoint.shutdown();
}

pub fn stop() {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(CLIENT_DEVICE_URL).unwrap();

    let mut reply = String::new();

    let request = "LOGGER/1.0 STOP".to_string();

    match socket.write_all(request.as_bytes()) {
        Ok(..) => println!("Send '{}'.", request),
        Err(err) => {
            println!("Client failed to send request '{}'.", err);
            return
        }
    }

    match socket.read_to_string(&mut reply) {
        Ok(_) => {
            println!("Recv '{}'.", reply);
            reply.clear()
        },
        Err(err) => {
            println!("Client failed to receive reply '{}'.", err);
            return
        }
    }
    endpoint.shutdown();
}

pub fn client() {
    trace!("Logger server task 'stop'");

    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(CLIENT_DEVICE_URL).unwrap();
    let mut count = 1u32;

    let mut reply = String::new();

    loop {
        let request = format!("Request #{}", count);

        match socket.write_all(request.as_bytes()) {
            Ok(..) => println!("Send '{}'.", request),
            Err(err) => {
                println!("Client failed to send request '{}'.", err);
                break
            }
        }

        match socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("Recv '{}'.", reply);
                reply.clear()
            },
            Err(err) => {
                println!("Client failed to receive reply '{}'.", err);
                break
            }
        }
        thread::sleep_ms(100);
        count += 1;
    }

    endpoint.shutdown();

}
