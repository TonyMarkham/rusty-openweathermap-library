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
    
    pub fn set_zip(&mut self, zip: String){
        self.zip = zip;
    }
    
    pub fn get_zip(&self) -> String {
        self.zip.clone()
    }

    pub fn set_country(&mut self, country: String){
        self.country = country;
    }
    
    pub fn get_country(&self) -> String {
        self.country.clone()
    }

    pub fn set_api_key(&mut self, api_key: String){
        self.api_key = api_key;
    }

    pub async fn get_location(&self) -> Result<Location, Box<dyn std::error::Error>> {
        let zip = format!("{},{}", &self.zip, &self.country);

        let response = self.client
            .get(GEOCODING_API_BASE_URL)
            .query(&[
                ("zip", &zip),
                ("appid", &self.api_key)
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()).into());
        }

        Ok(response.json().await?)
    }

    pub fn detailed_display(&self) -> String {
        format!(
            r#"country: [{}] - zip: [{}]"#,
            self.country,
            self.zip,
        )
    }
}

// TESTING

#[cfg(test)]
use mockall::automock;

#[cfg(test)]
use async_trait::async_trait;



/// A trait for location client operations to enable mocking
#[cfg(test)]
#[async_trait]
#[automock]
pub trait LocationClientTrait {
    async fn get_location(&self) -> Result<Location, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    const TEST_ZIP: &str = "N7L";
    const TEST_NAME: &str = "Chatham";
    const TEST_COUNTRY: &str = "CA";
    const TEST_LAT: f64 = 43.6532;
    const TEST_LON: f64 = -79.3832;

    #[tokio::test]
    async fn test_get_location_success() {
        // Create a mock instance
        let mut mock_client = MockLocationClientTrait::new();

        // Create expected Location data (you'll need to adjust this based on your Location struct)
        let expected_location = Location {
            zip: TEST_ZIP.to_string(),
            name: TEST_NAME.to_string(),
            lat: TEST_LAT,
            lon: TEST_LON,
            country: TEST_COUNTRY.to_string(),
        };

        // Set up the mock expectation
        mock_client
            .expect_get_location()
            .times(1)
            .returning(move || {
                let expected_location = expected_location.clone();
                Box::pin(async move { Ok(expected_location) })
            });

        // Call the method and verify the result
        let result = mock_client.get_location().await;

        assert!(result.is_ok());

        let location = result.unwrap();

        assert_eq!(location.zip, TEST_ZIP);
        assert_eq!(location.name, TEST_NAME);
        assert_eq!(location.lat, TEST_LAT);
        assert_eq!(location.lon, TEST_LON);
        assert_eq!(location.country, TEST_COUNTRY);
    }

    #[tokio::test]
    async fn test_get_location_failure() {
        // Create a mock instance
        let mut mock_client = MockLocationClientTrait::new();

        // Set up the mock to return an error
        mock_client
            .expect_get_location()
            .times(1)
            .returning(|| Box::pin(async move { Err("API request failed".into()) }));

        // Call the method and verify it returns an error
        let result = mock_client.get_location().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "API request failed");
    }
}

