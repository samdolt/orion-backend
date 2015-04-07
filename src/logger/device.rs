

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
