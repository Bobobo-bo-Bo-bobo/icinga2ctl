pub mod add_ack;
pub mod add_comment;
pub mod add_downtime;
pub mod cmd_help;
pub mod del_ack;
pub mod del_comment;
pub mod del_downtime;
pub mod generate_ticket;
pub mod reschedule_check;
pub mod status;
pub mod version;

use crate::constants;
use crate::usage;

pub fn show() {
    usage::version::show();
    println!(
        "Usage: {} -c <cfg>|--config=<cfg> [-d|--debug] [-h|--help] [-v|--version] <command> [<command_options>...]

    -c <cfg>        Read configuration from <cfg>
    --config <cfg>  This parameter is mandatory

    -d              Enable debug output
    --debug

    -h              Shows this text
    --help

    -v              Show version information
    --version

  Commands:
",
        constants::NAME
    );

    usage::add_ack::show();
    usage::add_comment::show();
    usage::add_downtime::show();
    usage::del_ack::show();
    usage::del_comment::show();
    usage::del_downtime::show();
    usage::generate_ticket::show();
    usage::reschedule_check::show();
    usage::status::show();
}
