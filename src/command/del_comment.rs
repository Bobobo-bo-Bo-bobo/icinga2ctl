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
    let mut flt_type = "";
    let mut filter = String::new();
    let mut author_str = String::new();
    let mut filter_str = String::new();

    if opt.is_present("help") {
        usage::version::show();
        usage::del_comment::show();
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

    let comment = match opt.value_of("comment_name") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    if hosts.is_empty() && services.is_empty() && comment.is_empty() {
        bail!("Neither comment name nor host/service filter provided");
    }

    if !(comment.is_empty() || hosts.is_empty() || services.is_empty()) {
        bail!("Provide either a comment name or a host/service filter, but not both");
    }

    if !hosts.is_empty() && services.is_empty() {
        flt_type = "Host";
        filter = format!("match(\\\"{}\\\", host.name)", hosts);
    }

    if hosts.is_empty() && !services.is_empty() {
        flt_type = "Service";
        filter = format!("match(\\\"{}\\\", service.name)", services);
    }

    if !hosts.is_empty() && !services.is_empty() {
        flt_type = "Service";
        filter = format!(
            "match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name)",
            hosts, services
        );
    }

    if !author.is_empty() {
        author_str = format!(",\"author\":\"{}\"", author);
    }

    if !filter.is_empty() {
        filter_str = format!(",\"filter\":\"{}\",\"type\":\"{}\"", filter, flt_type);
    }

    let payload = format!(
        "{{\"comment\":\"{comment}\"{author}{filter}}}",
        comment = comment,
        author = author_str,
        filter = filter_str,
    );

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_DEL_COMMENT,
            )
            .as_str(),
        )
        .body(payload)
        .send()?;

    match req.status() {
        StatusCode::NOT_FOUND => {
            println!("Comment {} does not exist", comment);
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
