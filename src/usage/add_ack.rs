pub fn show() {
    println!(
        "    add-ack - add acknowledgement

        -C <comment>                Set acknowledgement comment
        --comment=<comment>         This option is mandatory.

        -H <host_object>            Set acknowledgement for host <host_object>
        --host=<host_object>

        -N                          Don't send notification
        --no-notification

        -S <service_object>         Set acknowledgement for service <service_object>
        --service=<service_object>

        -a <author>                 Set author of acknowledgement
        --author=<author>           Default: current user

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
