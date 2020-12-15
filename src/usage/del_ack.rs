pub fn show() {
    println!(
        "    del-ack - remove acknowledgement

        -H <host_object>            Delete acknowledgement for host <host_object>
        --host=<host_object>

        -S <service_object>         Delete acknowledgement for service <service_object>
        --service=<service_object>

        -c                          Delete acknowledgement for host/services with CRITICAL state
        --critical

        -h                          Show this text
        --help

        -u                          Delete acknowledgement for host/services with UNKNOWN state
        --unknown

        -w                          Delete acknowledgement for host/services with WARNING state
        --warning
"
    );
}
