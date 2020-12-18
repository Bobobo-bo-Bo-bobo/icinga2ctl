use std::error::Error;

use crate::usage;

pub fn show(cmd: &str) -> Result<(), Box<dyn Error>> {
    match cmd {
        "add-ack" => {
            usage::version::show();
            usage::add_ack::show();
        }
        "add-comment" => {
            usage::version::show();
            usage::add_comment::show();
        }
        "add-downtime" => {
            usage::version::show();
            usage::add_downtime::show();
        }
        "del-ack" => {
            usage::version::show();
            usage::del_ack::show();
        }
        "del-comment" => {
            usage::version::show();
            usage::del_comment::show();
        }
        "del-downtime" => {
            usage::version::show();
            usage::del_downtime::show();
        }
        "generate-ticket" => {
            usage::version::show();
            usage::generate_ticket::show();
        }
        "reschedule-check" => {
            usage::version::show();
            usage::reschedule_check::show();
        }
        "status" => {
            usage::version::show();
            usage::status::show();
        }
        _ => {
            bail!("Unknown command {}", cmd);
        }
    };
    Ok(())
}
