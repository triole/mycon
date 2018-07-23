#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Read;

use serde_json;
use serde_derive;


pub struct IP {
    ip_current: String,
    ip_services: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IPServices {
    ip_services: Vec<String>,
}

impl IP{
    pub fn init() {
        let ips = Self::deserialize();
        println!("{:?}", ips);

    }

    fn deserialize() -> IPServices{
        let bytes = include_bytes!("services_list.json");
        let ip_services: IPServices = serde_json::from_slice(bytes).unwrap();
        println!("{:#?}", ip_services);
        return ip_services;
    }
}


// fn read_ipservices() -> Vec<String> {
    // let filename = get_filepath_formation_json(formation);
    // let fb = include_bytes!("ipservices.json");

    // let filename = "ipservices.json";
    // let s = read_js_to_string(&filename);
    // let ipservices: Vec<String> = from_str(&s).unwrap();
    // return ipservices;
// }

fn read_js_to_string(filename: &str) -> String {
    let mut s = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    return s;
}

#[cfg(test)]
mod tests {

    use services_file_reader::IP;

    #[test]
    fn test_ip_init(){
        IP::init();
    }

}
