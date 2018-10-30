use clap;
use clap::{App, ArgMatches};

use serde_yaml;
use std::env::current_exe;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use argparse;

#[derive(Clone, Debug, Default)]
pub struct Env {
    config: Config,
    args: Args,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
struct Config {
    ip_retrieval_services: Vec<String>,
}

#[derive(Clone, Debug, Default)]
struct Args {
    verbose: bool,
}

impl Env {
    pub fn init() -> Env {
        Env::default()
    }

    pub fn fill(&self) -> Env {
        let yaml = load_yaml!("args.yaml");
        let args = clap::App::from_yaml(yaml).get_matches();

        return Env {
            config: self.read_config(),
            args: Args {
                verbose: argparse::bool(&args, "verbose"),
            },
        };
    }

    fn read_config(&self) -> Config {
        let filename = self.config_file();
        let path = Path::new(&filename);
        let display = path.display();

        let mut file = match File::open(&filename) {
            Err(err) => {
                println!("Couldn't open {}: {}", display, err.description());
                process::exit(0x0101);
            }
            Ok(file) => file,
        };

        let mut s = String::new();
        file.read_to_string(&mut s)
            .expect("something went wrong reading the file");

        let config: Config = serde_yaml::from_str(&s).unwrap();
        return config;
    }

    fn curexe(&self) -> String {
        let c = current_exe().unwrap();
        return c.into_os_string().into_string().unwrap();
    }

    fn config_file(&self) -> String {
        return self.curexe().to_string() + ".yaml";
    }
}
