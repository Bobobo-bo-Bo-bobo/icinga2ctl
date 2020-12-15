pub fn show() {
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
