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
    fetch::channelize(env.config.ip_retrieval_services, env.args);
}
