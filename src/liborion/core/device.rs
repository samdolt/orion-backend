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

/// Internal representation of a device
///
/// # Example
///
/// ```
/// use orion::core::Device;
///
/// let device = Device::with_slug("port@node.driver");
/// ```
#[derive(Debug)]
pub struct Device {
    slug  : String,
    port  : String,
    node  : String,
    driver: String,
}

impl Device {

    /// Construct a new Device for given port, node and driver
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::Device;
    ///
    /// let device = Device::new("port", "node", "driver").unwrap();
    ///
    /// ```
    ///
    /// # Failures
    ///
    /// `port`, `node` and `driver` string must only contains alphanumerics,
    /// `-` or `_` characters.
    ///
    /// ```
    /// use orion::core::Device;
    ///
    /// // Invalid device
    /// assert!( Device::new("port$", "node", "driver").is_none() );
    ///
    /// // Valid device
    /// let device = Device::new("port-10", "node_2", "drivers1").unwrap();
    /// ```
    pub fn new(port: &str, node: &str, driver: &str) -> Option< Device > {

        let device = Device {
            slug   : format!("{}@{}.{}", port, node, driver),
            port   : port.to_string(),
            node   : node.to_string(),
            driver : driver.to_string(),
        };

        match device.is_valid() {
            true  => Some(device),
            false => None,
        }
    }

    /// Construct a new `Device` for a given slug
    ///
    /// A device slug has this form : `"port@node.driver"
    ///
    /// # Example
    ///
    /// ```
    /// use orion::core::Device;
    ///
    /// let device = Device::with_slug("port@node.driver").unwrap();
    /// ```
    ///
    /// # Failures
    ///
    /// -  A slug must respect this form : `port@node.driver`.
    /// - `port`, `node` and `driver` must only contains alphanumerics,
    ///   `-` or `_` characters.
    ///
    /// ```
    /// use orion::core::Device;
    ///
    /// // Invalid slug
    /// assert!( Device::with_slug("port.10@node$1.driver").is_none() );
    ///
    /// // Valid slug
    /// let device = Device::with_slug("port-10@node_2.drivers1").unwrap();
    /// ```
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
                        None    => unreachable!(),
                     },
            node   : match data.at(2) {
                        Some(x) => x.to_string(),
                        None    => unreachable!(),
                     },
            driver : match data.at(3) {
                        Some(x) => x.to_string(),
                        None    => unreachable!(),
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
        "PORT",
        "NODE",
        "DRIVER",
    );
    assert!( dev1.is_some() );

    // Invalid new device
    let dev2 = Device::new(
        "PORT-INV",
        "NODE@",
        "DRIVERS.",
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

#[test]
fn test_device_get_slug() {
    let dev1 = Device::with_slug("port@node.driver").unwrap();
    assert_eq!(dev1.get_slug(), "port@node.driver");

    let dev2 = Device::new(
        "port",
        "node",
        "driver",
    ).unwrap();

    assert_eq!(dev2.get_slug(), "port@node.driver");
}

#[test]
fn test_device_get_port_node_and_driver() {
    let dev1 = Device::with_slug("port@node.driver").unwrap();
    assert_eq!(dev1.get_port(), "port");
    assert_eq!(dev1.get_node(), "node");
    assert_eq!(dev1.get_driver(), "driver");

    let dev2 = Device::new(
        "port",
        "node",
        "driver",
    ).unwrap();
    assert_eq!(dev2.get_port(), "port");
    assert_eq!(dev2.get_node(), "node");
    assert_eq!(dev2.get_driver(), "driver");
}



