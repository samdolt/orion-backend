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

pub static COPYRIGHT: &'static str = "
Copyright © 2015 Samuel Dolt <samuel@dolt.ch>
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>

This is free software: you are free to change or redistribute it.
The is NO WARRANTY, to the extent permitted by law.
";

pub static INVALID_TIMESTAMP: &'static str = "
Invalid timestamp - Timestamp must be a valid IETF RFC3339 string.

Example:

  - 1985-04-12T23:20:50.52Z

    Represents 20 minutes and 50.52 seconds after the 23rd hour of
    April 12th, 1985 in UTC

More info at `https://www.ietf.org/rfc/rfc3339.txt`
";

pub static INVALID_VALUE: &'static str = "
Invalid value - Value should represent one or more measurements

Example:

  - 9[V]
  - 9[V] 3[A] 5[K]

Valid unit:
  - [V]  for Volt
  - [A]  for Ampere
  - [Ω]  for Ohm
  - [W]  for Watt
  - [K]  for Kelvin
  - [s]  for second
  - [kg] for Kilogram
";

pub static INVALID_DEVICE: &'static str = "
Invalid device - Device should be port@node.driver

Example:

  - temp1@core-isa-000.lm-sensors
  - temp_0@arduino100.arduino_usb
";
