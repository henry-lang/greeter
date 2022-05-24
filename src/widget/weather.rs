use reqwest::blocking::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Search {
    ByCity { city: String },
    ByZip { zip: String },
    ByCoordinates { lat: f64, lon: f64 },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Units {
    Standard,
    Imperial,
    Metric,
}

#[derive(Debug, Deserialize)]
pub struct WeatherConfig {
    api_key: String,

    #[serde(default = "WeatherConfig::default_units")]
    units: Units,

    #[serde(flatten)]
    search: Search,
}

impl WeatherConfig {
    fn default_units() -> Units {
        Units::Standard
    }
}

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    #[serde()]
    temp: f64,
}

pub fn display_weather_widget(config: &WeatherConfig) {
    let data = match &config.search {
        Search::ByCity { city } => get(format!(
            "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={}",
            config.api_key
        )),
        Search::ByZip { zip } => get(format!(
            "https://api.openweathermap.org/data/2.5/weather?q={zip}&appid={}",
            config.api_key
        )),
        Search::ByCoordinates { lat, lon } => get(format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={}",
            config.api_key
        )),
    };
    println!("{:?}", data.unwrap().json::<WeatherData>().unwrap())
}
