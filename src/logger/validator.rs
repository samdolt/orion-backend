




use regex;

use chrono::DateTime;

pub trait OrionLoggerValidator {

    fn is_RFC3339_timestamp(&self) -> bool;
    fn is_RFC3339_UTC_timestamp(&self) -> bool;
    fn is_measurement(&self) -> bool;
    fn is_measurements_list(&self) -> bool;
}

impl OrionLoggerValidator for String {

    fn is_RFC3339_timestamp(&self) -> bool {
        match DateTime::parse_from_rfc3339(self.as_str()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn is_RFC3339_UTC_timestamp(&self) -> bool {
        true
    }
    
    fn is_measurement(&self) -> bool {
    
        let re = regex!(r"^[-+]?[0-9]*\.?[0-9]*[\[](V|A|Ω|W|K|s|kg)[\]]$");
        
        if re.is_match(self.as_str()) == false{
            debug!("is_measurement failed on {}", self);
            return false;
        }
        
        true
    }
    
    fn is_measurements_list(&self) -> bool {
        for measurement in self.as_str().split(' ') {
            if measurement.to_string().is_measurement() == false {
                debug!("is_measurements_list failed on {}", self);
                return false;
            }
        }
        
        true
    }

}

