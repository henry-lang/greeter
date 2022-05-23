use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::widget::Widget;

#[derive(Deserialize)]
pub struct Config {
    name: Option<String>,
    widgets: Vec<Widget>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        match fs::read_to_string(&path) {
            Ok(data) => serde_json::from_str::<Self>(&data).unwrap(),
            _ => Self::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: None,
            widgets: vec![],
        }
    }
}
