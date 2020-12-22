use crate::configuration;
use crate::constants;

use reqwest::header;
use std::error::Error;
use std::fs;

pub fn build_client(
    cfg: &configuration::Configuration,
    over: &str,
) -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let mut bld = reqwest::blocking::ClientBuilder::new().use_native_tls();

    let mut head = header::HeaderMap::new();
    head.insert(
        header::ACCEPT,
        header::HeaderValue::from_str("application/json").unwrap(),
    );
    head.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json").unwrap(),
    );
    head.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(constants::user_agent().as_str()).unwrap(),
    );

    if !over.is_empty() {
        head.insert(
            "X-HTTP-Method-Override",
            header::HeaderValue::from_str(over).unwrap(),
        );
    }

    if cfg.insecure_ssl {
        bld = bld.danger_accept_invalid_certs(true);
        bld = bld.danger_accept_invalid_hostnames(true);
    // Adding a CA certificate is pointless if we don't validate the server certificate at all
    } else if !cfg.ca_file.is_empty() {
        let ca = fs::read(&cfg.ca_file)?;
        let ca_cert = reqwest::Certificate::from_pem(&ca)?;
        bld = bld.add_root_certificate(ca_cert);
    }

    // Note: Although RequestBuilder can handle basic auth, authentication using a client
    //       certificate must be configured in the ClientBuilder. So we just add the
    //       corresponding Authorization header here
    match cfg.auth {
        constants::AUTH_USER => {
            head.insert(
                header::AUTHORIZATION,
                header::HeaderValue::from_str(
                    format!(
                        "Basic {}",
                        base64::encode(format!("{}:{}", cfg.auth_user, cfg.auth_password))
                    )
                    .as_str(),
                )
                .unwrap(),
            );
        }
        constants::AUTH_CERT => {
            let raw_cert = fs::read(&cfg.auth_cert)?;
            let pkcs12 = reqwest::Identity::from_pkcs12_der(&raw_cert, &cfg.auth_cert_password)?;
            bld = bld.identity(pkcs12);
        }
        _ => {
            panic!("BUG: Invalid authentication method");
        }
    };

    bld = bld.default_headers(head);

    let cli = bld.build().unwrap();
    Ok(cli)
}
