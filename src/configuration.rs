use crate::constants;

use ini::Ini;
use std::env;
use std::error::Error;
use std::str::FromStr;
use url::Url;

pub struct Configuration {
    pub url: String,
    pub ca_file: String,
    pub insecure_ssl: bool,
    pub auth: u8,
    pub auth_user: String,
    pub auth_password: String,
    pub auth_cert: String,
    pub auth_cert_password: String,
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
    Ok(cfg)
}

pub fn get_configuration(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let mut config = Configuration {
        url: String::new(),
        ca_file: String::new(),
        insecure_ssl: false,
        auth: constants::AUTH_USER,
        auth_user: String::new(),
        auth_password: String::new(),
        auth_cert: String::new(),
        auth_cert_password: String::new(),
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
                    "user" => {
                        config.auth_user = value.to_string();
                    }
                    "password" => {
                        config.auth_password = value.to_string();
                    }
                    "auth_cert" => {
                        config.auth_cert = value.to_string();
                    }
                    "auth_cert_password" => {
                        config.auth_cert_password = value.to_string();
                    }
                    "insecure_ssl" => {
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
                        config.url = normalize_url(&value.to_string())?;
                    }
                    _ => {}
                }
            }
        }
    }

    if let Err(e) = validate_configuration(&config) {
        return Err(e);
    }

    Ok(config)
}

fn normalize_url(u: &str) -> Result<String, Box<dyn Error>> {
    let parsed = Url::parse(u)?;

    let scheme = parsed.scheme();

    if scheme != "https" {
        bail!("Invalid scheme: Icinga2 only supports https")
    }

    let host = match parsed.host_str() {
        Some(v) => v,
        None => {
            bail!("Missing host in URL");
        }
    };
    let port = match parsed.port() {
        Some(v) => v,
        None => 443, // scheme is always https
    };

    let n = format!("{}://{}:{}", scheme, host, port);
    Ok(n)
}

fn validate_configuration(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if cfg.url.is_empty() {
        bail!("Missing Icinga2 URL");
    }

    match cfg.auth {
        constants::AUTH_USER => {
            if cfg.auth_user.is_empty() {
                bail!("User authentication enabled but no user set");
            }

            if cfg.auth_password.is_empty() {
                bail!("User authentication enabled but no password set");
            }
        }
        constants::AUTH_CERT => {
            if cfg.auth_cert.is_empty() {
                bail!("Client certificate authentication enabled but no certificate file set");
            }
        }
        _ => {
            bail!("Invalid authentication method or authentication method not set");
        }
    };
    Ok(())
}
