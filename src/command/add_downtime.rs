use crate::configuration;
use crate::constants;
use crate::json_data;
use crate::request;
use crate::usage;

use chrono::DateTime;
use http::StatusCode;
use std::error::Error;
use std::str::FromStr;

pub fn run(
    cfg: &configuration::Configuration,
    opt: &clap::ArgMatches,
) -> Result<(), Box<dyn Error>> {
    let mut start_stamp_str = String::new();
    let mut end_stamp_str = String::new();
    let mut all_services = false;
    let mut fixed = false;
    let mut filter = String::new();
    let mut dwn_type = "";
    let mut duration_str = String::new();
    let mut trigger_str = String::new();

    if opt.is_present("help") {
        usage::add_downtime::show();
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
        None => bail!("Downtime comment is mandatory"),
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
                None => bail!("Can't get effective user name from operating system. Please provide the author of the downtime using the --author option"),
            }
        }
    };

    let start_str = match opt.value_of("start") {
        Some(v) => v.to_string(),
        None => String::new(),
    };
    let end_str = match opt.value_of("end") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let duration = match opt.value_of("duration") {
        Some(v) => match i32::from_str(v) {
            Ok(vv) => {
                if vv > 0 {
                    vv
                } else {
                    bail!("A duration of 0 or less is rather pointless");
                }
            }
            Err(e) => {
                bail!("Can't convert duration into a number: {}", e);
            }
        },
        None => -1,
    };

    if opt.is_present("all_services") {
        all_services = true;
    }

    if opt.is_present("fixed") {
        fixed = true;
    }

    let trigger = match opt.value_of("trigger") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let child_opts = match opt.value_of("child_opts") {
        Some(v) => match v.to_lowercase().as_str() {
            "downtimenochildren" => "DowntimeNoChildren",
            "downtimetriggeredchildren" => "DowntimeTriggeredChildren",
            "downtimenontriggeredchildren" => "DowntimeNonTriggeredChildren",
            _ => {
                bail!("Invalid child option {}", v);
            }
        },
        None => "DowntimeNoChildren",
    };

    if !start_str.is_empty() {
        match DateTime::parse_from_rfc3339(start_str.as_str()) {
            Ok(v) => start_stamp_str = v.format("%s").to_string(),
            Err(e) => bail!("Can't parse start time as RFC3339 time: {}", e),
        };
    };
    if !end_str.is_empty() {
        match DateTime::parse_from_rfc3339(end_str.as_str()) {
            Ok(v) => end_stamp_str = v.format("%s").to_string(),
            Err(e) => bail!("Can't parse end time as RFC3339 time: {}", e),
        };
    };

    if !fixed && duration < 0 {
        bail!("Flexible downtime must have a duration");
    }

    if all_services && !services.is_empty() {
        bail!("Adding downtime for all services and adding downtime for specific services are mutually exclusive");
    }

    if hosts.is_empty() && services.is_empty() {
        bail!("Provide a host and/or service for downtime addition");
    }

    if !hosts.is_empty() && services.is_empty() {
        dwn_type = "Host";
        filter = format!("match(\\\"{}\\\", host.name)", hosts);
    }

    if hosts.is_empty() && !services.is_empty() {
        dwn_type = "Service";
        filter = format!("match(\\\"{}\\\", service.name)", services);
    }

    if !hosts.is_empty() && !services.is_empty() {
        dwn_type = "Service";
        filter = format!(
            "match(\\\"{}\\\", host.name) && match(\\\"{}\\\", service.name)",
            hosts, services
        );
    }

    if duration > 0 {
        duration_str = format!(",\"duration\":\"{}\"", duration);
    }

    if !trigger.is_empty() {
        trigger_str = format!(",\"trigger_name\":\"{}\"", trigger);
    }

    let payload = format!("{{\"type\":\"{dwntype}\",\"filter\":\"{filter}\",\"author\":\"{author}\",\"comment\":\"{comment}\",\"start_time\":\"{start_time}\",\"end_time\":\"{end_time}\",\"fixed\":{fixed}{duration},\"all_services\":{all_svc}{trigger},\"child_options\":\"{child_options}\"}}",
                          all_svc=all_services,
                          author=author,
                          child_options=child_opts,
                          comment=comment,
                          duration=duration_str,
                          dwntype=dwn_type,
                          end_time=end_stamp_str,
                          filter=filter,
                          fixed=fixed,
                          start_time=start_stamp_str,
                          trigger=trigger_str,
                          );

    let req = request::build_client(cfg, "")?
        .post(
            format!(
                "{url}{path}",
                url = cfg.url,
                path = constants::ICINGA2_ADD_DOWNTIME,
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
