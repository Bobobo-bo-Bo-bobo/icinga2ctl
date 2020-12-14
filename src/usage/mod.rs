pub mod add_ack;
pub mod add_downtime;
pub mod del_ack;
pub mod del_downtime;
pub mod status;
pub mod version;

use crate::constants;
use crate::usage;

pub fn show() {
    usage::version::show();
    println!(
        "Usage: {} -c <cfg>|--config=<cfg> [-h|--help] [-v|--version] <command> [<command_options>...]

    -c <cfg>        Read configuration from <cfg>
    --config <cfg>  This parameter is mandatory

    -h              Shows this text
    --help

    -v              Show version information
    --version
",
        constants::NAME
    );
}
