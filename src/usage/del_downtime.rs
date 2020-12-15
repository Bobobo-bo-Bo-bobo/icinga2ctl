pub fn show() {
    println!(
        "    del-downtime - remove downtime
        -A <author>                 Set downtime removal author to <author>
        --author=<author>

        -D <downtime>               Name of downtime to be removed (as it is returned by add-downtime).
        --downtime=<downtime>       Downtime name and host/service objects are mutually exclusive

        -H <host_object>            Limit downtime removal to host <host_object>
        --host=<host_object>        Downtime name and host/service objects are mutually exclusive

        -S <service_object>         Limit downtime removal to  service <service_object>
        --service=<service_object>  Downtime name and host/service objects are mutually exclusive

        -h                          Show this text
        --help
"
    );
}
