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

use std::ops::Drop;
use std::io::{Write,Read};
use std::io::Result as IOResult;
use nanomsg::Socket;
use nanomsg::Endpoint;
use nanomsg::Protocol;
use nanomsg::Result as NanoResult;

const CLIENT_DEVICE_URL: &'static str = "ipc:///tmp/orion_logger_front.ipc";

pub struct Channel {

    socket: Socket,
    endpoint: Endpoint,
}

impl Channel {
    pub fn new () -> NanoResult<Channel> {
        let mut socket = try!(Socket::new(Protocol::Req) );
        let mut endpoint = try!( socket.connect(CLIENT_DEVICE_URL) );

        Ok(
            Channel{
                socket: socket,
                endpoint: endpoint,
            }
        )
    }

    pub fn request(&mut self, data: String) -> IOResult<String> {
        let mut reply = String::new();

        try!( self.socket.write_all(data.as_bytes()) );
        try!( self.socket.read_to_string(&mut reply) );

        Ok( reply )
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.endpoint.shutdown();
    }
}

#[test]
fn test_channel_new() {
    let channel = Channel::new().unwrap();
}
