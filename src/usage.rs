use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2020 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
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

pub fn show_usage_status() {
    show_version();
    println!(
        "   status - show Icinga2 host/service status

        -H <host_object>            Show status of host <host_object>
        --host=<host_object>

        -N                          Don't colorise output
        --no-color

        -S <service_object>         Show status of service <service_object>
        --service=<service_object>

        -c                          Show only host/services with CRITICAL state
        --critical

        -h                          Show this text
        --help

        -o                          Show only host/services with OK state
        --ok

        -u                          Show only host/services with UNKNOWN state
        --unknown

        -w                          Show only host/services with WARNING state
        --warning
"
    );
}
