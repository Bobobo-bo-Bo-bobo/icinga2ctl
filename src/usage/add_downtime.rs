use crate::usage;

pub fn show() {
    usage::version::show();
    println!(
        "    add-downtime - schedule a downtime

        -A                          If host downtime is added, add downtime for all services too.
        --all-services              Only valid for host downtimes

        -C <comment>                Set downtime comment.
        --comment=<comment>         This option is mandatory.

        -H <host_object>            Show status of host <host_object>
        --host=<host_object>

        -S <service_object>         Show status of service <service_object>
        --service=<service_object>

        -a <author>                 Set author of downtime
        --author=<author>           Default: current user

        -c <childopts>              Schedule child downtime.
        --child=<childopts>         <childopts> can be one of:
                                        no         - don't schedule child downtimes
                                                     This is the default behavior.
                                        trigger    - Add child downtimes triggered by the downtime to be added
                                        no-trigger - Add non-triggered child downtimes
                                    See downtime documentation for futher information.

        -d <sec>                    Set downtime duration for flexible downtime
        --duration=<sec>            This option is mandatory for flexible downtimes.
                                    See downtime documentation for further information.

        -e <time>                   Set end time of downtime.
        --end=<time>                <time> must be in the format as specified in RFC3339,
                                    e.g. 2010-11-09T12:54:11Z for UTC or 2020-11-09T23:11:12+02:00 for
                                    a time with time zone.
                                    This option is mandatory.

        -f                          Add fixed downtime instead of a flexible downtime.
        --fixed                     See downtime documentation for further information.

        -h                          Show this text
        --help

        -s <time>                   Set start time of downtime
        --start=<time>              <time> must be in the format as specified in RFC3339,
                                    e.g. 2010-11-09T12:54:11Z for UTC or 2020-11-09T23:11:12+02:00 for
                                    a time with time zone.
                                    This option is mandatory.

        -t <trigger>                Add trigger for triggered downtime.
        --trigger=<trigger>         See downtime documentation for futher information.

"
    );
}
