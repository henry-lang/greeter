use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged, rename_all(deserialize = "snake_case"))]
pub enum Search {
    ByCity { city: String },
    ByCoordinates { lat: f64, long: f64 },
    ByZip { zip: String },
}

#[derive(Debug, Deserialize)]
pub struct WeatherConfig {
    api_key: String,

    #[serde(flatten)]
    search: Search,
}

pub fn display_weather_widget(config: &WeatherConfig) {}
