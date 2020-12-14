use crate::usage;

pub fn show() {
    usage::version::show();
    println!(
        "    del-downtime - remove downtime
        -A <author>                 Set downtime removal author to <author>
        --author=<author>

        -D <downtime>               Name of downtime to be removed (as it is returned by add-downtime).
        --downtime=<downtime>       This option is mandatory.

        -H <host_object>            Limit downtime removal to host <host_object>
        --host=<host_object>

        -S <service_object>         Limit downtime removal to  service <service_object>
        --service=<service_object>

        -h                          Show this text
        --help
"
    );
}
