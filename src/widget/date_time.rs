use chrono::Local;
use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct DateTimeConfig {
    #[serde(default)]
    format: Cow<'static, str>,
}

impl Default for DateTimeConfig {
    fn default() -> Self {
        Self {
            format: Cow::Borrowed("%D %M %Y"),
        }
    }
}

pub fn display_date_time_widget(config: &DateTimeConfig) {
    println!("{}", Local::now().format(config.format.as_ref()));
}
