use super::types::Location;

// https://api.openweathermap.org/geo/1.0/zip?zip=N7L,CA&appid={api_key}

const GEOCODING_API_BASE_URL: &str = "https://api.openweathermap.org/geo/1.0/zip";

/// A client for accessing location data via a geocoding API using a zip code and country code.
///
/// This struct encapsulates an HTTP client and required parameters such as the zip code,
/// country code, and API key. It provides methods to update these parameters and an async
/// method to fetch location information from the remote API.
///
/// # Fields
/// - `client`: The HTTP client used to send requests.
/// - `api_key`: API key for authenticating requests.
/// - `zip`: Zip code for the location query.
/// - `country`: Country code for the location query.
pub struct LocationClient {
    client: reqwest::Client,
    api_key: String,
    zip: String,
    country: String,
}

impl LocationClient {
    pub fn new(zip: String, country: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            zip,
            country,
            api_key,
        }
    }

    pub fn set_zip(&mut self, zip: String) {
        self.zip = zip;
    }

    pub fn get_zip(&self) -> String {
        self.zip.clone()
    }

    pub fn set_country(&mut self, country: String) {
        self.country = country;
    }

    pub fn get_country(&self) -> String {
        self.country.clone()
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }

    pub async fn get_location(&self) -> Result<Location, Box<dyn std::error::Error>> {
        let zip = format!("{},{}", &self.zip, &self.country);

        let response = self
            .client
            .get(GEOCODING_API_BASE_URL)
            .query(&[("zip", &zip), ("appid", &self.api_key)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }

    pub fn detailed_display(&self) -> String {
        format!(r#"country: [{}] - zip: [{}]"#, self.country, self.zip,)
    }
}
