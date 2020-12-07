#[macro_use]
extern crate simple_error;

mod configuration;
mod constants;
mod usage;

use getopts::Options;
use std::env;
use std::process;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();

    options.optopt("c", "config", "Configuration file", "config_file");
    options.optflag("h", "help", "Show help text");
    options.optflag("v", "version", "Show version");

    let opt = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments: {}", e);
            process::exit(1);
        }
    };

    if opt.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    if opt.opt_present("v") {
        usage::show_version();
        process::exit(0);
    }

    let config_file = match opt.opt_str("c") {
        Some(v) => v,
        None => {
            let v = match configuration::get_default_user_config_file() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            };
            v
        }
    };
    let config = match configuration::get_configuration(config_file.as_str()) {
        Ok(v) => { v },
        Err(e) => {
            eprintln!("Error: Can't parse configuration file {}: {}", config_file, e);
            process::exit(1);
        },
    };

    if opt.free.is_empty() {
        eprintln!("Error: Missing command");
        usage::show_usage();
        process::exit(1);
    }

    println!("> {:#?}", opt.free);
}
