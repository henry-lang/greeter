mod weather;

use serde::Deserialize;

use weather::WeatherSearch;

#[derive(Deserialize)]
pub enum Widget {
    Time { format: String },
    Weather(WeatherSearch),
}
