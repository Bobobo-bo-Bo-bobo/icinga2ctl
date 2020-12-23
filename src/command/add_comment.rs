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
    let mut cmt_type = "";
    let mut filter = String::new();

    if opt.is_present("help") {
        usage::version::show();
        usage::add_comment::show();
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
    let comment = match opt.value_of("comment") {
        Some(v) => v.to_string(),
        None => bail!("Comment comment is mandatory"),
    };

    let author = match opt.value_of("author") {
        Some(v) => v.to_string(),
        None => {
            match users::get_effective_username() {
                Some(u) => {
                    match u.into_string() {
                        Ok(us) => us,
                        Err(e) => bail!("Can't convert effective user name into a String: {:?}", e)
                    }
                }
                None => bail!("Can't get effective user name from operating system. Please provide the author of the comment using the --author option"),
            }
        }
    };

    if hosts.is_empty() && services.is_empty() {
        bail!("Provide a host and/or service for comment addition");
    }

    if !hosts.is_empty() && services.is_empty() {
        cmt_type = "Host";
        filter = format!("match(\\\"{}\\\", host.name)", hosts);
    }

    if hosts.is_empty() && !services.is_empty() {
        cmt_type = "Service";
        filter = format!("match(\\\"{}\\\", service.name)", services);
    }

    if !hosts.is_empty() && !services.is_empty() {
        cmt_type = "Service";
        filter = format!(
            "match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name)",
            hosts, services
        );
    }

    let payload = format!("{{\"type\":\"{cmttype}\",\"filter\":\"{filter}\",\"author\":\"{author}\",\"comment\":\"{comment}\"}}",
                          author=author,
                          comment=comment,
                          cmttype=cmt_type,
                          filter=filter,
                          );

    if cfg.debug {
        eprintln!("HTTP method: POST");
        eprintln!("URL: {}{}", cfg.url, constants::ICINGA2_ADD_COMMENT);
        eprintln!("Payload: {}", payload);
    }

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_ADD_COMMENT,
            )
            .as_str(),
        )
        .body(payload)
        .send()?;

    // Note: If all hosts/services objectes selected are already acknowledged, a HTTP 500 is
    //       returned because the acknowledgement can't be processed:
    //         "A status in the range of 500 generally means that there was a server-side problem and
    //          Icinga 2 is unable to process your request."
    //          (https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#http-statuses)
    if req.status() != StatusCode::OK && req.status() != StatusCode::INTERNAL_SERVER_ERROR {
        let reason = match req.status().canonical_reason() {
            Some(v) => v,
            _ => "None",
        };
        bail!(
            "Invalid status code received, exepected HTTP status code 200 or 500, got \"{}\" instead",
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
        match ar.name {
            Some(v) => {
                println!("{}: {}", v, ar.status);
            }
            None => {
                println!("{}", ar.status);
            }
        };
    }

    Ok(())
}
