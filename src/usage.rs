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

        -a <type>                   Only show states with <ack> acknowledgement type
        --ack=<type>                    yes    - acknowledged
                                        no     - not acknowledged
                                        sticky - sticky acknowledgment
                                    Note: Hosts/services in OK state have no acknowledgment set,
                                          except for sticky acknowledgements.

        -c                          Show host/services with CRITICAL state
        --critical

        -h                          Show this text
        --help

        -o                          Show host/services with OK state
        --ok

        -u                          Show host/services with UNKNOWN state
        --unknown

        -w                          Show host/services with WARNING state
        --warning
"
    );
}

pub fn show_usage_add_ack() {
    show_version();
    println!(
        "    add-ack - add acknowledgement

        -H <host_object>            Show status of host <host_object>
        --host=<host_object>

        -N                          Don't send notification
        --no-notification

        -S <service_object>         Show status of service <service_object>
        --service=<service_object>

        -a <author>                 Set author of acknowledgement
        --author=<author>           Default: current user

        -C <comment>                Set acknowledgement comment
        --comment=<comment>         This option is mandatory.

        -c                          Acknowledge host/services with CRITICAL state
        --critical

        -e <time>                   Set acknowledgement expiration to <time>
        --expire=<time>             <expire> must be in the format as specified in RFC3339,
                                    e.g. 2010-11-09T12:54:11Z for UTC or 2020-11-09T23:11:12+02:00 for
                                    a time with time zone.

        -h                          Show this text
        --help

        -p                          Set persitent acknowledgement
        --persistent

        -s                          Set sticky acknowledgement
        --sticky

        -u                          Acknowledge host/services with UNKNOWN state
        --unknown

        -w                          Acknowledge host/services with WARNING state
        --warning
"
    );
}
