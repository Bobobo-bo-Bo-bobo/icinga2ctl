#[macro_use]
extern crate simple_error;

mod command;
mod configuration;
mod constants;
mod json_data;
mod request;
mod usage;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let mut config_file = String::new();

    let options = App::new(constants::NAME)
        .version(constants::VERSION)
        .about("Interact with Icinga 2 instance")
        .args(&[
            Arg::with_name("config_file")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Configuration file"),
            Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Show help text"),
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Show version information"),
        ])
        .subcommand(
            SubCommand::with_name("status")
                .about("Show status of host and service objects")
                .args(&[
                    Arg::with_name("host_object")
                        .short("H")
                        .long("host")
                        .takes_value(true)
                        .help("Show status of host objects"),
                    Arg::with_name("service_object")
                        .short("S")
                        .long("service")
                        .takes_value(true)
                        .help("Show status of service objects"),
                    Arg::with_name("help")
                        .short("h")
                        .long("help")
                        .help("Show help text for status command"),
                ]),
        )
        .get_matches();

    if options.is_present("help") {
        usage::show_usage();
        process::exit(0);
    }

    if options.is_present("version") {
        usage::show_version();
        process::exit(1);
    }

    if let Some(v) = options.value_of("config_file") {
        config_file = v.to_string();
    }

    if config_file.is_empty() {
        config_file = match configuration::get_default_user_config_file() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
    }

    let config = match configuration::get_configuration(config_file.as_str()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Error: Can't parse configuration file {}: {}",
                config_file, e
            );
            process::exit(1);
        }
    };

    match options.subcommand() {
        ("status", Some(m)) => {
            if let Err(e) = command::status(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Error: No command provided");
            usage::show_usage();
            process::exit(1);
        }
    };
}
