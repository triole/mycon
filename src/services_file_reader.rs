#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Read;

use serde_derive;
use serde_json;

#[derive(Debug)]
pub struct IP {
    pub ip_current: String,
    pub ip_services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IPServices {
    ip_services: Vec<String>,
}

impl IP {
    pub fn init() -> IP {
        let ip_services = Self::get_ip_services();
        // println!("{:?}",ip_services.ip_services);
        let ip = IP {
            ip_current: "unknown".to_string(),
            ip_services: ip_services,
        };
        return ip;
    }

    fn get_ip_services() -> Vec<String> {
        let bytes = include_bytes!("services_list.json");
        let ips: IPServices = serde_json::from_slice(bytes).unwrap();
        let ip_services = ips.ip_services;
        return ip_services;
    }
}

// --- tests
#[cfg(test)]
mod tests {

    use services_file_reader::IP;

    #[test]
    fn test_ip_init() {
        IP::init();
    }

}
