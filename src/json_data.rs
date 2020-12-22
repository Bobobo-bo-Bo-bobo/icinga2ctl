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
    pub acknowledgement: f64,
    pub display_name: String,
    pub host_name: Option<String>,
    pub last_check_result: LastCheckResult,
    pub name: Option<String>,
    pub state: f64,
    pub state_type: f64,
}

#[derive(Serialize, Deserialize)]
pub struct LastCheckResult {
    pub active: bool,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct Icinga2ActionReply {
    pub results: Vec<Icinga2ActionReplyResult>,
}

#[derive(Serialize, Deserialize)]
pub struct Icinga2ActionReplyResult {
    code: f64,
    pub name: Option<String>,
    pub status: String,
    pub ticket: Option<String>,
}
