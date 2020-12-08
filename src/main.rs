#[macro_use]
extern crate simple_error;

mod command;
mod configuration;
mod constants;
mod json_data;
mod request;
mod usage;
mod util;

use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let mut config_file = String::new();

    let options = App::new(constants::NAME)
        .version(constants::VERSION)
        .about("Interact with Icinga 2 instance")
        .args(&[
            Arg::with_name("config_file")
                .help("Read configuration from <config_file>")
                .short("c")
                .long("config")
                .takes_value(true),
            Arg::with_name("help")
                .help("Show this text")
                .short("h")
                .long("help"),
            Arg::with_name("version")
                .help("Show version information")
                .short("v")
                .long("version"),
        ])
        .subcommand(
            SubCommand::with_name("status")
                .about("Show status of host and service objects")
                .args(&[
                    Arg::with_name("host_object")
                        .help("Show status of host <host_object>")
                        .short("H")
                        .long("host")
                        .takes_value(true),
                    Arg::with_name("service_object")
                        .help("Show status of service <service_object>")
                        .short("S")
                        .long("service")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("no-color")
                        .help("Don't colorise output")
                        .short("N")
                        .long("no-color"),
                    Arg::with_name("ok")
                        .help("Show only host/services with OK state")
                        .short("o")
                        .long("ok"),
                    Arg::with_name("warning")
                        .help("Show only host/services with WARNING state")
                        .short("w")
                        .long("warning"),
                    Arg::with_name("critical")
                        .help("Show only host/services with CRITICAL state")
                        .short("c")
                        .long("critical"),
                    Arg::with_name("unknown")
                        .help("Show only host/services with UNKNOWN state")
                        .short("u")
                        .long("unknown"),
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
