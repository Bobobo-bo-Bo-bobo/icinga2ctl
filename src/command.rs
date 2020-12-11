use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;
use crate::util;

use chrono::DateTime;
use http::StatusCode;
use std::collections::HashMap;
use std::error::Error;

pub fn del_ack(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut ack_type = "";
    let mut filter = String::new();
    let mut state_map = HashMap::new();

    if opt.is_present("help") {
        usage::show_usage_del_ack();
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
        bail!("Provide a host and/or service for status display");
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
            build_state_filter(&ack_type.to_lowercase(), &state_map)
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

pub fn add_ack(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut sticky = false;
    let mut persistent = false;
    let mut notify = true;
    let mut expire_stamp_str = String::new();
    let mut ack_type = "";
    let mut filter = String::new();
    let mut state_map = HashMap::new();

    if opt.is_present("help") {
        usage::show_usage_add_ack();
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
        None => bail!("Acknowledgement comment is mandatory"),
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
                None => bail!("Can't get effective user name from operating system. Please provide the author of the acknowledgement using the --author option"),
            }
        }
    };

    let expire_str = match opt.value_of("expire") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    if opt.is_present("sticky") {
        sticky = true;
    }

    if opt.is_present("persistent") {
        persistent = true;
    }

    if opt.is_present("no-notification") {
        notify = false;
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

    if !expire_str.is_empty() {
        match DateTime::parse_from_rfc3339(expire_str.as_str()) {
            Ok(v) => expire_stamp_str = format!("\"expiry\":{},", v.format("%s").to_string()),
            Err(e) => bail!("Can't parse expiration time as RFC3339 time: {}", e),
        };
    };

    if hosts.is_empty() && services.is_empty() {
        bail!("Provide a host and/or service for status display");
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
            build_state_filter(&ack_type.to_lowercase(), &state_map)
        );
    }

    let payload = format!("{{\"type\":\"{acktype}\",\"filter\":\"{filter}\",\"author\":\"{author}\",\"comment\":\"{comment}\",{expiry}\"sticky\":{sticky},\"notify\":{notify},\"persistent\":{persistent}}}",
                          acktype=ack_type,
                          filter=filter,
                          author=author,
                          comment=comment,
                          sticky=sticky,
                          expiry=expire_stamp_str,
                          notify=notify,
                          persistent=persistent);

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_ACK_PROBLEM,
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

pub fn status(
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
        usage::show_usage_status();
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

        let state_filter = build_state_filter("host", &state_map);
        let ack_filter = build_ack_filter("host", ack);

        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", host.name) && {} && {}\"}}",
            hosts, state_filter, ack_filter,
        );
        attrs = "attrs=name&attrs=display_name&attrs=last_check_result&attrs=state&attrs=acknowledgement";
    }

    if hosts.is_empty() && !services.is_empty() {
        // Show services for all hosts
        obj = constants::ICINGA2_OBJ_SERVICE;

        let state_filter = build_state_filter("service", &state_map);
        let ack_filter = build_ack_filter("service", ack);

        filter = format!(
            "{{\"filter\":\"match(\\\"{}\\\", service.name) && {} && {}\"}}",
            services, state_filter, ack_filter,
        );
        attrs = "attrs=display_name&attrs=host_name&attrs=last_check_result&attrs=state&attrs=acknowledgement";
    }

    if !hosts.is_empty() && !services.is_empty() {
        // Show services for hosts
        obj = constants::ICINGA2_OBJ_SERVICE;

        let state_filter = build_state_filter("service", &state_map);
        let ack_filter = build_ack_filter("service", ack);

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

fn build_state_filter(n: &str, m: &HashMap<String, String>) -> String {
    if !m.is_empty() {
        let mut v = Vec::new();
        for key in m.keys() {
            v.push(format!("{}.state {}", n, key));
        }
        return format!("({})", v.join(" || "));
    }
    format!("{}.state >= 0", n)
}

fn build_ack_filter(n: &str, ack: i8) -> String {
    match ack {
        constants::ICINGA2_ACK_NONE => format!("{}.acknowledgement == 0", n),
        constants::ICINGA2_ACK_ACK => format!("{}.acknowledgement == 1", n),
        constants::ICINGA2_ACK_STICKY => format!("{}.acknowledgement == 2", n),
        _ => format!("{}.acknowledgement >= 0", n),
    }
}
