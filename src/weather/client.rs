use super::types::WeatherResponse;
use crate::location::Location;

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct WeatherClient {
    client: reqwest::Client,
    location: Location,
    units: String,
    api_key: String,
}

impl WeatherClient {
    pub fn new(location: Location, units: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            location,
            units: units.clone(),
            api_key,
        }
    }

    pub async fn get_current_weather(&self) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let response = self.client
            .get(WEATHER_API_BASE_URL)
            .query(&[
                ("lat", self.location.lat.to_string()),
                ("lon", self.location.lon.to_string()),
                ("units", self.units.to_string()),
                ("appid", self.api_key.clone())
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }
}