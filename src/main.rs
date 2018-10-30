#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
extern crate clap;

// my packages
mod argparse;
mod env;
mod fetch;

fn main() {
    let env = env::Env::init();
    env.fill();

    // let ip = services_file_reader::IP::init();
    // let config = config::read(&env::config_file());
    println!("{:#?}", env);

    // fetch::channelize(ip.ip_services);
}
