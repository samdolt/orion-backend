



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
Invalide value - Value should represent one or more measurements

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

