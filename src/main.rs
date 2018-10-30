#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
extern crate clap;

// my packages
mod argparse;
mod channelize;
mod env;
mod util;

fn main() {
    let env = env::Env::init();
    channelize::work(env.config.ip_retrieval_services, env.args);
}
