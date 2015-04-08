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

#[derive(Debug)]
pub struct Device {
    slug  : String,
    port  : String,
    node  : String,
    driver: String,
}

impl Device {

    pub fn new(port: String, node: String, driver: String) -> Option< Device > {

        let device = Device {
            slug   : format!("{}@{}.{}", port, node, driver),
            port   : port,
            node   : node,
            driver : driver,
        };

        match device.is_valid() {
            true  => Some(device),
            false => None,
        }
    }

    pub fn with_slug(slug : &str) -> Option< Device > {

        let re = regex!(r"^([\w-]*)@([\w-]*).([\w-]*)$");

        let data = match re.captures(slug) {
            Some(x) => x,
            None    => return None,
        };


        let device = Device {
            slug   : slug.to_string(),
            port   : match data.at(1) {
                        Some(x) => x.to_string(),
                        None    => return None,
                     },
            node   : match data.at(2) {
                        Some(x) => x.to_string(),
                        None    => return None,
                     },
            driver : match data.at(3) {
                        Some(x) => x.to_string(),
                        None    => return None,
                     },
        };

        Some(device)
    }

    fn is_valid(&self) -> bool {
        let re = regex!(r"^[\w-]*@[\w-]*.[\w-]*$");

        if re.is_match(self.slug.as_str()) == false {
            debug!("is_valid failed on {:?}", self);
            return false;
        }

        true
    }

    pub fn get_slug<'a>(&'a self) -> &'a str {
        return self.slug.as_str()
    }

    pub fn get_port<'a>(&'a self) -> &'a str {
        return self.port.as_str()
    }

    pub fn get_node<'a>(&'a self) -> &'a str {
        return self.node.as_str()
    }

    pub fn get_driver<'a>(&'a self) -> &'a str {
        return self.driver.as_str()
    }
}


#[test]
fn test_device_new() {

    // Valid new device with A-Z
    let dev1 = Device::new(
        "PORT".to_string(),
        "NODE".to_string(),
        "DRIVER".to_string(),
    );
    assert!( dev1.is_some() );

    // Invalid new device
    let dev2 = Device::new(
        "PORT-INV".to_string(),
        "NODE@".to_string(),
        "DRIVERS.".to_string(),
    );
    assert!( dev2.is_none() );
}

#[test]
fn test_device_with_slug() {
    // Valid new device
    let dev1 = Device::with_slug("port@node.driver");
    
    assert!( dev1.is_some() );

    // Invalid new device
    let dev2 = Device::with_slug("port@node.driver.driver");

    assert!( dev2.is_none() );
}
