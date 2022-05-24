use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::widget::Widget;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: Option<String>,

    #[serde(default)]
    pub debug: bool,

    #[serde(default)]
    pub widgets: Vec<Widget>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str::<Self>(&data).unwrap_or_else(|err| {
                println!(
                    "Found error while parsing config, using default instead: {}",
                    err
                );
                Self::default()
            }),
            _ => Self::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            name: None,
            widgets: vec![],
        }
    }
}
