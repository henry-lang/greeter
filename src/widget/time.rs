use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
pub struct TimeConfig {
    format: Cow<'static, str>,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            format: Cow::Borrowed("%D %M %Y"),
        }
    }
}
