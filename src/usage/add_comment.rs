pub fn show() {
    println!(
        "    add-comment - add a comment

        -C <comment>                Set comment comment.
        --comment=<comment>         This option is mandatory.

        -H <host_object>            Add comment for host <host_object>
        --host=<host_object>

        -S <service_object>         Add comment for service <service_object>
        --service=<service_object>

        -a <author>                 Set author of comment
        --author=<author>           Default: current user

        -h                          Show this text
        --help

"
    );
}
