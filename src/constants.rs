pub const NAME: &str = "icinga2ctl";
pub const VERSION: &str = "0.7.0-20201215";
const PROJECT_URL: &str = "https://git.ypbind.de/cgit/icinga2ctl";

pub const AUTH_USER: u8 = 0x01;
pub const AUTH_CERT: u8 = 0x02;

pub const ICINGA2_OBJ_SERVICE: &str = "/v1/objects/services";
pub const ICINGA2_OBJ_HOST: &str = "/v1/objects/hosts";
pub const ICINGA2_ADD_ACK: &str = "/v1/actions/acknowledge-problem";
pub const ICINGA2_DEL_ACK: &str = "/v1/actions/remove-acknowledgement";
pub const ICINGA2_ADD_DOWNTIME: &str = "/v1/actions/schedule-downtime";
pub const ICINGA2_DEL_DOWNTIME: &str = "/v1/actions/remove-downtime";

pub const ICINGA2_ACK_NONE: i8 = 0;
pub const ICINGA2_ACK_ACK: i8 = 1;
pub const ICINGA2_ACK_STICKY: i8 = 2;

pub fn user_agent() -> String {
    format!("{}/{} (+{})", NAME, VERSION, PROJECT_URL)
}
