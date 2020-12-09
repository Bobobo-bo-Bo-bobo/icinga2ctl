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

pub fn print_state(s: &str, st: &str, a: f64, color: bool) {
    let ack = !matches!(a.round() as i64, 0);

    if color {
        match st {
            "OK" => {
                println!("{}", Green.paint(s));
            }
            "WARNING" => {
                if ack {
                    println!("{}", Yellow.paint(s));
                } else {
                    println!("{}", Yellow.bold().underline().paint(s));
                }
            }
            "CRITICAL" => {
                if ack {
                    println!("{}", Red.paint(s));
                } else {
                    println!("{}", Red.bold().underline().paint(s));
                }
            }
            "UNKNOWN" => {
                if ack {
                    println!("{}", Purple.paint(s));
                } else {
                    println!("{}", Purple.bold().underline().paint(s));
                }
            }
            _ => {
                println!("{}", s);
            }
        };
    } else {
        println!("{}", s);
    }
}
