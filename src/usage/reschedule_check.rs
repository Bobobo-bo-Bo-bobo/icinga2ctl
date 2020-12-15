pub fn show() {
    println!(
"   reschedule-check - reschedule Icinga2 check
 
        -A <type>                   Only reschedule checks with <ack> acknowledgement type
        --ack=<type>                    yes    - acknowledged
                                        no     - not acknowledged
                                        sticky - sticky acknowledgment
                                    Note: Hosts/services in OK state have no acknowledgment set,
                                          except for sticky acknowledgements.

		-H <host_object>			Reschedule checks for host <host_object>
        --host=<host_object>

        -S <service_object>			Reschedule checks for service <service_object>
        --service=<service_object>
 
        -a <time>                   Instead of rerun check immediately, run check at <time>
        --at <time>                 <time> must be in the format as specified in RFC3339,
                                    e.g. 2010-11-09T12:54:11Z for UTC or 2020-11-09T23:11:12+02:00 for
                                    a time with time zone.
 
        -c                          Reschedule host/services checks with CRITICAL state
        --critical

        -f							Force recheck
        --force

        -h                          Show this text
        --help

        -o                          Reschedule host/services checks with OK state
        --ok

        -u                          Reschedule host/services checks with UNKNOWN state
        --unknown

        -w                          Reschedule host/services checks with WARNING state
        --warning

 "
     );
}
