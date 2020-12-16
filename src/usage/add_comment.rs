pub fn show() {
    println!(
        "    add-comment - schedule a comment

        -C <comment>                Set comment comment.
        --comment=<comment>         This option is mandatory.

        -H <host_object>            Show status of host <host_object>
        --host=<host_object>

        -S <service_object>         Show status of service <service_object>
        --service=<service_object>

        -a <author>                 Set author of comment
        --author=<author>           Default: current user

        -h                          Show this text
        --help

"
    );
}
