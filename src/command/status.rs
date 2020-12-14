use crate::command;
use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;
use crate::util;

use http::StatusCode;
use std::collections::HashMap;
use std::error::Error;

pub fn run(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut obj: &str = "";
    let mut attrs: &str = "";
    let mut filter = String::new();
    let mut color = true;
    let mut state_map = HashMap::new();
    let mut ack = -1;

    if opt.is_present("help") {
        usage::status::show();
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

    if opt.is_present("no-color") {
        color = false;
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
        obj = constants::ICINGA2_OBJ_HOST;

        let state_filter = command::filter::build_state_filter("host", &state_map);
        let ack_filter = command::filter::build_ack_filter("host", ack);

        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && {} && {}\"}}",
            hosts, state_filter, ack_filter,
        );
        attrs = "attrs=name&attrs=display_name&attrs=last_check_result&attrs=state&attrs=acknowledgement";
    }

    if hosts.is_empty() && !services.is_empty() {
        // Show services for all hosts
        obj = constants::ICINGA2_OBJ_SERVICE;

        let state_filter = command::filter::build_state_filter("service", &state_map);
        let ack_filter = command::filter::build_ack_filter("service", ack);

        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", service.name) && {} && {}\"}}",
            services, state_filter, ack_filter,
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state&attrs=acknowledgement";
    }

    if !hosts.is_empty() && !services.is_empty() {
        // Show services for hosts
        obj = constants::ICINGA2_OBJ_SERVICE;

        let state_filter = command::filter::build_state_filter("service", &state_map);
        let ack_filter = command::filter::build_ack_filter("service", ack);

        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name) && {} && {}\"}}",
            hosts, services, state_filter, ack_filter,
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state&attrs=acknowledgement";
    }

    let req = request::build_client(cfg, "GET")?
        .post(
            format!(
                "{url}{path}?{attrs}",
                url = cfg.url,
                path = obj,
                attrs = attrs
            )
            .as_str(),
        )
        .body(filter)
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

    let status_result: json_data::Icinga2Status = match serde_json::from_str(&raw.as_str()) {
        Ok(v) => v,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    for r in status_result.results {
        // if host_name is set, display_name is the name of the service
        match r.attrs.host_name {
            Some(v) => {
                util::print_state(
                    &format!(
                        "{host}: {service}: {status}: {output}",
                        host = v,
                        service = r.attrs.display_name,
                        output = r.attrs.last_check_result.output.replace("\n", " "),
                        status = util::state_to_string(r.attrs.state)
                    ),
                    util::state_to_string(r.attrs.state).as_str(),
                    r.attrs.acknowledgement,
                    color,
                );
            }
            None => {
                if let Some(v) = r.attrs.name {
                    util::print_state(
                        &format!(
                            "{host}: {status}: {output}",
                            host = v,
                            output = r.attrs.last_check_result.output.replace("\n", " "),
                            status = util::state_to_string(r.attrs.state)
                        ),
                        util::state_to_string(r.attrs.state).as_str(),
                        r.attrs.acknowledgement,
                        color,
                    );
                }
            }
        }
    }
    Ok(())
}
