use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;

use http::StatusCode;
use std::error::Error;

pub fn status(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut obj: &str = "";
    let mut attrs: &str = "";
    let mut filter = String::new();

    if opt.is_present("help") {
        usage::show_usage_status();
        return Ok(());
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
        filter = format!("{{\"filter\":\"match(\\\"{}\\\", host.name)\"}}", hosts);
        attrs = "attrs=name&attrs=display_name&attrs=last_check_result&attrs=state";
    }

    if hosts.is_empty() && !services.is_empty() {
        // Show services for all hosts
        obj = constants::ICINGA2_OBJ_SERVICE;
        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", service.name)\"}}",
            services
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state";
    }

    if !hosts.is_empty() && !services.is_empty() {
        // Show services for hosts
        obj = constants::ICINGA2_OBJ_SERVICE;
        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name)\"}}",
            hosts, services
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
                println!(
                    "{host}: {service}: {status}: {output}",
                    host = v,
                    service = r.attrs.display_name,
                    output = r.attrs.last_check_result.output,
                    status = json_data::state_to_string(r.attrs.state)
                );
            }
            None => {
                if let Some(v) = r.attrs.name {
                    println!(
                        "{host}: {status}: {output}",
                        host = v,
                        output = r.attrs.last_check_result.output,
                        status = json_data::state_to_string(r.attrs.state)
                    );
                }
            }
        }
    }
    Ok(())
}
