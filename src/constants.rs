pub const NAME: &str = "icinga2ctl";
pub const VERSION: &str = "1.0.0-20201207";
const PROJECT_URL: &str = "https://git.ypbind.de/cgit/icinga2ctl";

pub const AUTH_USER: u8 = 0x01;
pub const AUTH_CERT: u8 = 0x02;

pub fn user_agent() -> String {
    format!("{}/{} (+{})", NAME, VERSION, PROJECT_URL)
}
