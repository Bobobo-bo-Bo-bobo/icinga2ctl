use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;

use http::StatusCode;
use std::error::Error;

pub fn run(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    if opt.is_present("help") {
        usage::version::show();
        usage::generate_ticket::show();
        return Ok(());
    }

    let cn = match opt.value_of("cn") {
        Some(v) => v,
        None => {
            bail!("Missing CN for ticket generation")
        }
    };

    let payload = format!("{{\"cn\":\"{}\"}}", cn);

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_GENERATE_TICKET,
            )
            .as_str(),
        )
        .body(payload)
        .send()?;

    if req.status() != StatusCode::OK {
        let reason = match req.status().canonical_reason() {
            Some(v) => v,
            _ => "None",
        };
        bail!(
            "Invalid status code received, exepected \"200 OK\", got \"{}\" instead",
            reason
        );
    }

    let raw = req.text()?;

    let action_result: json_data::Icinga2ActionReply = match serde_json::from_str(&raw.as_str()) {
        Ok(v) => v,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    for ar in action_result.results {
        match ar.ticket {
            Some(v) => {
                println!("{}: {}", v, ar.status);
            }
            None => {
                println!("-: {}", ar.status);
            }
        };
    }

    Ok(())
}
