use crate::command;
use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;

use http::StatusCode;
use std::collections::HashMap;
use std::error::Error;

pub fn run(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut ack_type = "";
    let mut filter = String::new();
    let mut state_map = HashMap::new();

    if opt.is_present("help") {
        usage::del_ack::show();
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

    if opt.is_present("warning") {
        state_map.insert("==1".to_string(), String::new());
    }
    if opt.is_present("critical") {
        state_map.insert("==2".to_string(), String::new());
    }
    if opt.is_present("unknown") {
        state_map.insert("==3".to_string(), String::new());
    }

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
                None => bail!("Can't get effective user name from operating system. Please provide the author of the acknowledgement using the --author option"),
            }
        }
    };

    if hosts.is_empty() && services.is_empty() {
        bail!("Provide a host and/or service for acknowledgement deletion");
    }

    if !hosts.is_empty() && services.is_empty() {
        ack_type = "Host";
        filter = format!(
            "match(\\\"{}\\\", host.name) && host.state_type == 1",
            hosts
        );
    }

    if hosts.is_empty() && !services.is_empty() {
        ack_type = "Service";
        filter = format!(
            "match(\\\"{}\\\", service.name) && service.state_type == 1",
            services
        );
    }

    if !hosts.is_empty() && !services.is_empty() {
        ack_type = "Service";
        filter = format!("match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name) && service.state_type == 1", hosts, services);
    }

    if !state_map.is_empty() {
        filter = format!(
            "{} && ({})",
            filter,
            command::filter::build_state_filter(&ack_type.to_lowercase(), &state_map)
        );
    }

    let payload = format!(
        "{{\"type\":\"{acktype}\",\"filter\":\"{filter}\",\"author\":\"{author}\"}}",
        acktype = ack_type,
        filter = filter,
        author = author
    );

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_DEL_ACK,
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
        println!("{}", ar.status);
    }

    Ok(())
}
