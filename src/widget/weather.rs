use serde::Deserialize;

#[derive(Deserialize)]
pub enum WeatherSearch {
    ByName(String),
    ByCoordinates { lat: f64, long: f64 },
    ByZip,
}
