use crate::command;
use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;

use chrono::DateTime;
use http::StatusCode;
use std::collections::HashMap;
use std::error::Error;

pub fn run(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut filter = String::new();
    let mut force = true;
    let mut state_map = HashMap::new();
    let mut ack = -1;
    let mut state_type = "";
    let mut at_str = String::new();
    let mut at_stamp_str = String::new();

    if opt.is_present("help") {
        usage::version::show();
        usage::reschedule_check::show();
        return Ok(());
    }

    if opt.is_present("ok") {
        state_map.insert("== 0".to_string(), String::new());
    }
    if opt.is_present("warning") {
        state_map.insert("==1".to_string(), String::new());
    }
    if opt.is_present("critical") {
        state_map.insert("==2".to_string(), String::new());
    }
    if opt.is_present("unknown") {
        state_map.insert("==3".to_string(), String::new());
    }

    if opt.is_present("force") {
        force = true;
    }

    if let Some(v) = opt.value_of("ack") {
        ack = match v {
            "yes" => constants::ICINGA2_ACK_ACK,
            "no" => constants::ICINGA2_ACK_NONE,
            "sticky" => constants::ICINGA2_ACK_STICKY,
            _ => {
                bail!("Invalid value for acknowledgement option: {}", v);
            }
        };
    };

    let at = match opt.value_of("at") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    if !at.is_empty() {
        match DateTime::parse_from_rfc3339(at.as_str()) {
            Ok(v) => at_str = v.format("%s").to_string(),
            Err(e) => bail!("Can't parse time as RFC3339 time: {}", e),
        };
    };

    let hosts = match opt.value_of("host_object") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let services = match opt.value_of("service_object") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    if hosts.is_empty() && services.is_empty() {
        bail!("Provide a host and/or service for status display");
    }

    if !hosts.is_empty() && services.is_empty() {
        // Show host status for hosts
        let state_filter = command::filter::build_state_filter("host", &state_map);
        let ack_filter = command::filter::build_ack_filter("host", ack);
        state_type = "Host";

        filter = format!(
            "\"filter\":\"match(\\\"{}\\\", host.name) && {} && {}\"",
            hosts, state_filter, ack_filter,
        );
    }

    if hosts.is_empty() && !services.is_empty() {
        // Show services for all hosts
        let state_filter = command::filter::build_state_filter("service", &state_map);
        let ack_filter = command::filter::build_ack_filter("service", ack);
        state_type = "Service";

        filter = format!(
            "\"filter\":\"match(\\\"{}\\\", service.name) && {} && {}\"",
            services, state_filter, ack_filter,
        );
    }

    if !hosts.is_empty() && !services.is_empty() {
        // Show services for hosts
        let state_filter = command::filter::build_state_filter("service", &state_map);
        let ack_filter = command::filter::build_ack_filter("service", ack);
        state_type = "Service";

        filter = format!(
            "\"filter\":\"match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name) && {} && {}\"",
            hosts, services, state_filter, ack_filter,
        );
    }

    if !at_str.is_empty() {
        at_stamp_str = format!(",\"next_check\":\"{}\"", at_str);
    };

    let payload = format!(
        "{{\"type\":\"{state_type}\",{filter},\"force\":{force}{at_str}}}",
        state_type = state_type,
        filter = filter,
        force = force,
        at_str = at_stamp_str,
    );

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_RESCHEDULE_CHECK,
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
        println!("{}", ar.status);
    }

    Ok(())
}
