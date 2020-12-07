use crate::constants;

use ini::Ini;
use std::env;
use std::error::Error;
use std::str::FromStr;

pub struct Configuration {
    url: String,
    ca_file: String,
    insecure_ssl: bool,
    auth: u8,
    auth_user: String,
    auth_password: String,
    auth_pubkey: String,
    auth_privkey: String,
}

pub fn get_default_user_config_file() -> Result<String, Box<dyn Error>> {
    let mut cfgbase = String::new();

    // for the XDG environment, see https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    for (key, value) in env::vars() {
        if key == "XDG_CONFIG_HOME" {
            cfgbase = value;
            break;
        }
        if key == "HOME" {
            cfgbase = value + "/.config";
        }
    }

    if cfgbase.is_empty() {
        bail!("Unable to get user configuration. Neither the environmant variable XDG_CONFIG_HOME nor HOME are set")
    }
    let cfg = cfgbase + "/icinga2ctl/config.ini";
    return Ok(cfg);
}

pub fn get_configuration(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let mut config = Configuration {
        url: String::new(),
        ca_file: String::new(),
        insecure_ssl: false,
        auth: constants::AUTH_USER,
        auth_user: String::new(),
        auth_password: String::new(),
        auth_pubkey: String::new(),
        auth_privkey: String::new(),
    };

    let cfg = Ini::load_from_file(f)?;

    for (section, properties) in &cfg {
        let section_name = section.expect("Section is not defined");

        if section_name == "icinga2" {
            for (key, value) in properties.iter() {
                match key {
                    "auth" => {
                        match value {
                            "user" => {
                                config.auth = constants::AUTH_USER;
                            }
                            "cert" => {
                                config.auth = constants::AUTH_CERT;
                            }
                            _ => {
                                bail!("Invalid authentication type {}", value)
                            }
                        };
                    }
                    "auth_user" => {
                        config.auth_user = value.to_string();
                    }
                    "auth_password" => {
                        config.auth_password = value.to_string();
                    }
                    "auth_pubkey" => {
                        config.auth_pubkey = value.to_string();
                    }
                    "auth_privkey" => {
                        config.auth_privkey = value.to_string();
                    }
                    "insecures_ssl" => {
                        config.insecure_ssl = match FromStr::from_str(value) {
                            Ok(v) => v,
                            Err(e) => {
                                bail!(
                                    "Can't convert value {} for {} into a boolean: {}",
                                    value,
                                    key,
                                    e
                                );
                            }
                        }
                    }
                    "ca_file" => {
                        config.ca_file = value.to_string();
                    }
                    "url" => {
                        config.url = value.to_string();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(config)
}
