use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Icinga2Status {
    pub results: Vec<Icinga2StatusResult>,
}

#[derive(Serialize, Deserialize)]
pub struct Icinga2StatusResult {
    pub attrs: StatusResultAttrs,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusResultAttrs {
    pub host_name: Option<String>,
    pub name: Option<String>,
    pub display_name: String,
    pub state: f64,
    pub last_check_result: LastCheckResult,
}

#[derive(Serialize, Deserialize)]
pub struct LastCheckResult {
    pub active: bool,
    pub output: String,
}

pub fn state_to_string(s: f64) -> String {
    let mut ststr = "???";

    if s == 0.0 {
        ststr = "OK";
    }
    if s == 1.0 {
        ststr = "WARNING";
    }
    if s == 2.0 {
        ststr = "CRITICAL";
    }
    if s == 3.0 {
        ststr = "UNKNOWN";
    }

    ststr.to_string()
}
