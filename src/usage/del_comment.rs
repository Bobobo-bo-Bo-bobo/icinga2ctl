pub fn show() {
    println!(
        "    del-comment - remove comment

        -A <author>                 Set comment removal author to <author>
        --author=<author>

        -C <comment>                Name of comment to be removed (as it is returned by add-comment).
        --comment=<comment>         Comment name and host/service objects are mutually exclusive

        -H <host_object>            Limit comment removal to host <host_object>
        --host=<host_object>        Comment name and host/service objects are mutually exclusive

        -S <service_object>         Limit comment removal to  service <service_object>
        --service=<service_object>  Comment name and host/service objects are mutually exclusive

        -h                          Show this text
        --help
"
    );
}
