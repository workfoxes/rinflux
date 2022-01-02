use log::LogLevel;

use crate::client::options::http::HttpOption;
use crate::client::options::write::WriteOption;

pub(crate) mod write;
pub(crate) mod http;

pub struct Option {
    log_level: LogLevel,
    write_option: WriteOption,
    http_option: HttpOption,
}

impl Default for Option {
    fn default() -> Self {
        Option {
            log_level: LogLevel::Info,
            write_option: WriteOption::default(),
            http_option: HttpOption::default()
        }
    }
}

impl Option {
    pub fn new(log_level: LogLevel, write_option: WriteOption, http_option: HttpOption) -> Self {
        Option {
            log_level,
            write_option,
            http_option
        }
    }
}

