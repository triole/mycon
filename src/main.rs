#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[macro_use]
extern crate clap;

// my packages
mod argparse;
mod channelize;
mod env;
mod fetch_ip;
mod print;
mod util;

fn main() {
    let env = env::Env::init();

    if env.args.check == true {
        println!("{}", "Checking ip retrieval services...");
        for url in env.config.ip_retrieval_services {
            let (status, body) = util::fetch_url(&url);
            println!("\nRequest      : {}", url);
            println!("Status code  : {}", status);
            println!("Response body: {}", body);
        }
    } else {
        let result = channelize::work(env.config.ip_retrieval_services, env.args);
        print::print(result);
    }
}
