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
            SubCommand::with_name("add-ack")
                .about("Add acknowledgement")
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
                    Arg::with_name("author")
                        .help("Acknowledgment author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("expire")
                        .help("Set expiration for acknowledgement")
                        .short("e")
                        .long("expire")
                        .takes_value(true),
                    Arg::with_name("sticky")
                        .help("Add sticky acknowledgement")
                        .short("s")
                        .long("sticky"),
                    Arg::with_name("persistent")
                        .help("Add persistent acknowledgement")
                        .short("p")
                        .long("persistent"),
                    Arg::with_name("no-notification")
                        .help("Don't send notification")
                        .short("N")
                        .long("no-notification"),
                    Arg::with_name("comment")
                        .help("Comment to add")
                        .short("C")
                        .long("comment")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
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
        .subcommand(
            SubCommand::with_name("add-comment")
                .about("Add comment")
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
                    Arg::with_name("author")
                        .help("Downtime author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("comment")
                        .help("Comment to add")
                        .short("C")
                        .long("comment")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("add-downtime")
                .about("Add downtime")
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
                    Arg::with_name("author")
                        .help("Downtime author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("all_services")
                        .help("All services")
                        .short("A")
                        .long("all-services"),
                    Arg::with_name("start")
                        .help("Set start time for downtime")
                        .short("s")
                        .long("start")
                        .takes_value(true),
                    Arg::with_name("end")
                        .help("Set end time for downtime")
                        .short("e")
                        .long("end")
                        .takes_value(true),
                    Arg::with_name("comment")
                        .help("Comment to add")
                        .short("C")
                        .long("comment")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("duration")
                        .help("Set downtime duration for flexible downtime")
                        .short("d")
                        .long("duration")
                        .takes_value(true),
                    Arg::with_name("child_opts")
                        .help("Schedule child downtime")
                        .short("c")
                        .long("child")
                        .takes_value(true),
                    Arg::with_name("fixed")
                        .help("Add fixed downtime instead of a flexible downtime")
                        .short("f")
                        .long("fixed"),
                    Arg::with_name("trigger")
                        .help("Add trigger for triggered downtime")
                        .short("t")
                        .long("trigger")
                        .takes_value(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("del-ack")
                .about("Remove acknowledgement")
                .args(&[
                    Arg::with_name("host_object")
                        .help("Delete acknowledgement of host <host_object>")
                        .short("H")
                        .long("host")
                        .takes_value(true),
                    Arg::with_name("service_object")
                        .help("Delete acknowledgement of service <service_object>")
                        .short("S")
                        .long("service")
                        .takes_value(true),
                    Arg::with_name("author")
                        .help("Acknowledgment author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("warning")
                        .help("Acknowledge only host/services with WARNING state")
                        .short("w")
                        .long("warning"),
                    Arg::with_name("critical")
                        .help("Acknowledge only host/services with CRITICAL state")
                        .short("c")
                        .long("critical"),
                    Arg::with_name("unknown")
                        .help("Acknowledge only host/services with UNKNOWN state")
                        .short("u")
                        .long("unknown"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("del-comment")
                .about("Remove comment")
                .args(&[
                    Arg::with_name("host_object")
                        .help("Remove comment limited to host <host_object>")
                        .short("H")
                        .long("host")
                        .takes_value(true),
                    Arg::with_name("service_object")
                        .help("Remove comment limited to service <service_object>")
                        .short("S")
                        .long("service")
                        .takes_value(true),
                    Arg::with_name("author")
                        .help("Comment removal author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("comment_name")
                        .help("Name of comment to remove")
                        .short("D")
                        .long("comment")
                        .takes_value(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("del-downtime")
                .about("Remove downtime")
                .args(&[
                    Arg::with_name("host_object")
                        .help("Remove downtime limited to host <host_object>")
                        .short("H")
                        .long("host")
                        .takes_value(true),
                    Arg::with_name("service_object")
                        .help("Remove downtime limited to service <service_object>")
                        .short("S")
                        .long("service")
                        .takes_value(true),
                    Arg::with_name("author")
                        .help("Downtime removal author")
                        .short("a")
                        .long("author")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("downtime_name")
                        .help("Name of downtime to remove")
                        .short("D")
                        .long("downtime")
                        .takes_value(true),
                ]),
        )
        .subcommand(
            SubCommand::with_name("generate-ticket")
                .about("Generate PKI ticket for Icinga2 auto signing")
                .args(&[
                    Arg::with_name("cn")
                        .help("Common name attribute of the host for which the ticket should be created")
                        .short("C")
                        .long("cn")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("reschedule-check")
                .about("Reschedule checks of host and service objects")
                .args(&[
                    Arg::with_name("host_object")
                        .help("Reschedule checks for host <host_object>")
                        .short("H")
                        .long("host")
                        .takes_value(true),
                    Arg::with_name("service_object")
                        .help("Reschedule checks for service <service_object>")
                        .short("S")
                        .long("service")
                        .takes_value(true),
                    Arg::with_name("help")
                        .help("Show this text")
                        .short("h")
                        .long("help"),
                    Arg::with_name("force")
                        .help("Force recheck")
                        .short("f")
                        .long("force"),
                    Arg::with_name("ok")
                        .help("Reschedule host/services checks with OK state")
                        .short("o")
                        .long("ok"),
                    Arg::with_name("warning")
                        .help("Reschedule host/services checks with WARNING state")
                        .short("w")
                        .long("warning"),
                    Arg::with_name("critical")
                        .help("Reschedule host/services checks with CRITICAL state")
                        .short("c")
                        .long("critical"),
                    Arg::with_name("unknown")
                        .help("Reschedule host/services checks with UNKNOWN state")
                        .short("u")
                        .long("unknown"),
                    Arg::with_name("ack")
                        .help("Only reschedule checks with <ack> acknowledgement type")
                        .short("A")
                        .long("ack")
                        .takes_value(true),
                    Arg::with_name("at")
                        .help("Instead of rerun check immediately, run check at <time>")
                        .short("a")
                        .long("at")
                        .takes_value(true),
                ]),
        )
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
                    Arg::with_name("ack")
                        .help("Show only states with an acknowledgment of <ack>")
                        .short("A")
                        .long("ack")
                        .takes_value(true),
                ]),
        )
        .get_matches();

    if options.is_present("help") {
        usage::show();
        process::exit(0);
    }

    if options.is_present("version") {
        usage::show();
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
        ("add-ack", Some(m)) => {
            if let Err(e) = command::add_ack::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("add-comment", Some(m)) => {
            if let Err(e) = command::add_comment::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("add-downtime", Some(m)) => {
            if let Err(e) = command::add_downtime::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("del-ack", Some(m)) => {
            if let Err(e) = command::del_ack::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("del-comment", Some(m)) => {
            if let Err(e) = command::del_comment::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("del-downtime", Some(m)) => {
            if let Err(e) = command::del_downtime::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("generate-ticket", Some(m)) => {
            if let Err(e) = command::generate_ticket::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("reschedule-check", Some(m)) => {
            if let Err(e) = command::reschedule_check::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        ("status", Some(m)) => {
            if let Err(e) = command::status::run(&config, &m) {
                println!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Error: No command provided");
            usage::show();
            process::exit(1);
        }
    };
}
