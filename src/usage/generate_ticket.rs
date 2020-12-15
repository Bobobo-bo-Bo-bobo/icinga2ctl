pub fn show() {
    println!(
"   generate-ticket - generate PKI ticket for Icinga2 auto signing
 
        -C <cn>                     Common name attribute of the host for which the ticket should be created
        --cn=<cn>                   This option is mandatory.

        -h                          Show this text
        --help

 "
     );
}
