use crate::usage;

pub fn show() {
    usage::version::show();
    println!(
        "    del-ack - remove acknowledgement

        -H <host_object>            Show status of host <host_object>
        --host=<host_object>

        -S <service_object>         Show status of service <service_object>
        --service=<service_object>

        -c                          Acknowledge host/services with CRITICAL state
        --critical

        -h                          Show this text
        --help

        -u                          Acknowledge host/services with UNKNOWN state
        --unknown

        -w                          Acknowledge host/services with WARNING state
        --warning
"
    );
}
