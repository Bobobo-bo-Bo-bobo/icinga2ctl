use crate::constants;
use std::collections::HashMap;

pub fn build_state_filter(n: &str, m: &HashMap<String, String>) -> String {
    if !m.is_empty() {
        let mut v = Vec::new();
        for key in m.keys() {
            v.push(format!("{}.state {}", n, key));
        }
        return format!("({})", v.join(" || "));
    }
    format!("{}.state >= 0", n)
}

pub fn build_ack_filter(n: &str, ack: i8) -> String {
    match ack {
        constants::ICINGA2_ACK_NONE => format!("{}.acknowledgement == 0", n),
        constants::ICINGA2_ACK_ACK => format!("{}.acknowledgement == 1", n),
        constants::ICINGA2_ACK_STICKY => format!("{}.acknowledgement == 2", n),
        _ => format!("{}.acknowledgement >= 0", n),
    }
}

