use super::types::WeatherResponse;
use crate::location::Location;

const WEATHER_API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

/// A client for interacting with a location-based geocoding API.
///
/// This struct encapsulates an HTTP client along with location query parameters,
/// including ZIP code, country code, and an API key for authentication.
/// It provides methods to update these parameters and to request location data
/// asynchronously from the geocoding service.
///
/// # Fields
/// - `client`: The underlying HTTP client used to send requests.
/// - `api_key`: API key for authenticating requests.
/// - `zip`: ZIP or postal code for location lookup.
/// - `country`: Country code (ISO 3166-1 alpha-2 format).
///
/// # Usage
/// Create via `LocationClient::new` with ZIP, country, and API key.
/// Use `get_location` to asynchronously fetch location details.
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
        let response = self
            .client
            .get(WEATHER_API_BASE_URL)
            .query(&[
                ("lat", self.location.lat.to_string()),
                ("lon", self.location.lon.to_string()),
                ("units", self.units.to_string()),
                ("appid", self.api_key.clone()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }
}
