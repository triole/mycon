#![allow(dead_code)]

// serde json
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// my packages
mod services_file_reader;
use  services_file_reader::IP;


fn main() {
    println!("{:?}", "Running application...");

    let ip = IP::init();
    println!("{:?}",ip);
}
