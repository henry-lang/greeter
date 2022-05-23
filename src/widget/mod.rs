mod time;
mod weather;

use serde::Deserialize;

use time::TimeConfig;
use weather::WeatherConfig;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Widget {
    Time(TimeConfig),
    Weather(WeatherConfig),
}
