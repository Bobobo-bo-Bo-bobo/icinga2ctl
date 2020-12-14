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
    let mut filter = String::new();
    let mut author_str = String::new();
    let mut filter_str = String::new();

    if opt.is_present("help") {
        usage::del_downtime::show();
        return Ok(());
    };

    let hosts = match opt.value_of("host_object") {
        Some(v) => v.to_string(),
        None => String::new(),
    };
    let services = match opt.value_of("service_object") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let author = match opt.value_of("author") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let downtime = match opt.value_of("downtime_name") {
        Some(v) => v.to_string(),
        None => {
            bail!("Downtime name is mandatody");
        }
    };

    if !hosts.is_empty() && services.is_empty() {
        filter = format!("match(\\\"{}\\\", host.name)", hosts);
    }

    if hosts.is_empty() && !services.is_empty() {
        filter = format!("match(\\\"{}\\\", service.name)", services);
    }

    if !hosts.is_empty() && !services.is_empty() {
        filter = format!(
            "match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name)",
            hosts, services
        );
    }

    if !author.is_empty() {
        author_str = format!(",\"author\":\"{}\"", author);
    }

    if !filter.is_empty() {
        filter_str = format!(",\"filter\":\"{}\"", filter);
    }

    let payload = format!(
        "{{\"downtime\":\"{downtime}\"{author}{filter}}}",
        downtime = downtime,
        author = author_str,
        filter = filter_str,
    );

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_DEL_DOWNTIME,
            )
            .as_str(),
        )
        .body(payload)
        .send()?;

    match req.status() {
        StatusCode::NOT_FOUND => {
            println!("Downtime {} does not exist", downtime);
            return Ok(());
        }
        StatusCode::OK => {}
        _ => {
            let reason = match req.status().canonical_reason() {
                Some(v) => v,
                _ => "None",
            };
            bail!(
                "Invalid status code received, exepected HTTP status code 200, got \"{}\" instead",
                reason
            );
        }
    };

    let raw = req.text()?;

    let action_result: json_data::Icinga2ActionReply = match serde_json::from_str(&raw.as_str()) {
        Ok(v) => v,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    for ar in action_result.results {
        println!("{}", ar.status);
    }

    Ok(())
}
