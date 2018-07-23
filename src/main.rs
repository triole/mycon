#![allow(dead_code)]

// serde json
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// my packages
mod requester;

mod services_file_reader;
use services_file_reader::IP;

fn main() {
    println!("{:#?}", "Running application...");

    let ip = IP::init();
    println!("{:#?}", ip);

    println!("{:#?}", "Making request...");
    let rs = requester::get_url(&ip.ip_services[0]);
    // println!("{:?}", rs);
}
