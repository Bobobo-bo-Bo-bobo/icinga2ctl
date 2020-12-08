use ansi_term::Colour::{Green, Purple, Red, Yellow};

pub fn state_to_string(s: f64) -> String {
    match s.round() as i64 {
        0 => "OK",
        1 => "WARNING",
        2 => "CRITICAL",
        3 => "UNKNOWN",
        _ => "???",
    }
    .to_string()
}

pub fn print_state(s: &str, st: &str, color: bool) {
    if color {
        match st {
            "OK" => {
                println!("{}", Green.paint(s));
            }
            "WARNING" => {
                println!("{}", Yellow.paint(s));
            }
            "CRITICAL" => {
                println!("{}", Red.paint(s));
            }
            "UNKNOWN" => {
                println!("{}", Purple.paint(s));
            }
            _ => {
                println!("{}", s);
            }
        };
    } else {
        println!("{}", s);
    }
}
