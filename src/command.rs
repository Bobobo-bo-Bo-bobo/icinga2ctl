use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;
use crate::util;

use http::StatusCode;
use std::error::Error;

pub fn status(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut obj: &str = "";
    let mut attrs: &str = "";
    let mut filter = String::new();
    let mut color = true;
    let mut state_filter = ">=0";

    if opt.is_present("help") {
        usage::show_usage_status();
        return Ok(());
    }

    if opt.is_present("ok") {
        state_filter = "==0";
    }
    if opt.is_present("warning") {
        state_filter = "==1";
    }
    if opt.is_present("critical") {
        state_filter = "==2";
    }
    if opt.is_present("unknown") {
        state_filter = "== 3";
    }

    if opt.is_present("no-color") {
        color = false;
    }

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
        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && host.state {}\"}}",
            hosts, state_filter
        );
        attrs = "attrs=name&attrs=display_name&attrs=last_check_result&attrs=state";
    }

    if hosts.is_empty() && !services.is_empty() {
        // Show services for all hosts
        obj = constants::ICINGA2_OBJ_SERVICE;
        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", service.name) && service.state {}\"}}",
            services, state_filter
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state";
    }

    if !hosts.is_empty() && !services.is_empty() {
        // Show services for hosts
        obj = constants::ICINGA2_OBJ_SERVICE;
        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name) && service.state {}\"}}",
            hosts, services, state_filter
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state";
    }

    let req = request::build_client(cfg)?
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
                        color,
                    );
                }
            }
        }
    }
    Ok(())
}
