mod date_time;
mod weather;

use serde::Deserialize;

use date_time::DateTimeConfig;
use weather::WeatherConfig;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub enum Widget {
    Time(DateTimeConfig),
    Weather(WeatherConfig),
}

impl Widget {
    pub fn display(&self) {
        use Widget::*;
        match self {
            Time(config) => date_time::display_date_time_widget(config),
            Weather(config) => weather::display_weather_widget(config),
        }
    }
}
