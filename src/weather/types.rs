use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::location::{Location, LocationClient};
use crate::weather::WeatherClient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Latitude in decimal degrees
    pub lat: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
    /// Weather condition ID (internal OpenWeatherMap identifier)
    pub id: i32,
    /// Group of weather parameters (Rain, Snow, Clouds, etc.)
    pub main: String,
    /// Weather condition description (e.g., "light rain", "clear sky")
    pub description: String,
    /// Weather icon ID for displaying weather icons
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Main {
    /// Current temperature
    pub temp: f64,
    /// Temperature perception by humans
    pub feels_like: f64,
    /// Minimum temperature at the moment (for large cities and urban areas)
    pub temp_min: f64,
    /// Maximum temperature at the moment (for large cities and urban areas)
    pub temp_max: f64,
    /// Atmospheric pressure in hPa
    pub pressure: i32,
    /// Humidity percentage
    pub humidity: i32,
    /// Atmospheric pressure on the sea level in hPa (optional)
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level in hPa (optional)
    pub grnd_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wind {
    /// Wind speed (units vary by API request: m/s for metric, mph for imperial)
    pub speed: f64,
    /// Wind direction in degrees (meteorological)
    pub deg: i32,
    /// Wind gust speed (optional, same units as speed)
    pub gust: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clouds {
    /// Cloudiness percentage (0-100%)
    pub all: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Visibility {
    /// Visibility distance in meters
    pub visibility: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    /// Internal parameter for data source
    #[serde(rename = "type")]
    pub sys_type: Option<i32>,
    /// Internal parameter for data source
    pub id: Option<i32>,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Sunrise time in Unix timestamp UTC
    pub sunrise: i64,
    /// Sunset time in Unix timestamp UTC
    pub sunset: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeatherResponse {
    /// Geographic coordinates of the location
    pub coord: Coord,
    /// Weather condition information (can be multiple conditions)
    pub weather: Vec<Weather>,
    /// Internal parameter for data source
    pub base: String,
    /// Main weather measurements (temperature, pressure, humidity, etc.)
    pub main: Main,
    /// Visibility information
    pub visibility: i32,
    /// Wind information
    pub wind: Wind,
    /// Cloud coverage information
    pub clouds: Clouds,
    /// Time of data calculation in Unix timestamp UTC
    pub dt: i64,
    /// System information (country, sunrise, sunset, etc.)
    pub sys: Sys,
    /// Timezone shift in seconds from UTC
    pub timezone: i32,
    /// City/location ID
    pub id: i64,
    /// City/location name
    pub name: String,
    /// Internal parameter for API response
    pub cod: i32,
}

impl WeatherResponse {
    pub fn detailed_display(&self, units: String) -> String {
        let temp_display = get_temperature_display(self.main.temp, &units);
        let wind_display = get_speed_display(self.wind.speed, &units);
        let mut info_main = "";
        let mut info_description = "";
        if let Some(weather_info) = self.weather.first(){
            info_main = &weather_info.main;
            info_description = &weather_info.description;
        }

        format!(
            r#"🌤️ Weather in {}
📍 Coordinates: ({}, {})
🌡️ Temperature: {}
💨 Wind: {} at {}°
☁️ Clouds: {}%
🌈 Conditions: {} ({})"#,
            self.name,
            self.coord.lat,
            self.coord.lon,
            temp_display,
            wind_display,
            self.wind.deg,
            self.clouds.all,
            info_main,
            info_description,
        )
    }
}

fn get_temperature_display(temp: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1}°C", temp),
        "imperial" => format!("{:.1}°F", temp),
        "standard" | _ => format!("{:.1}°K", temp),
    }
}

fn get_speed_display(speed: f64, units: &str) -> String {
    match units {
        "metric" => format!("{:.1} m/s", speed),
        "imperial" => format!("{:.1} mph", speed),
        "standard" | _ => format!("{:.1} m/s", speed),
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherRequestWasm {
    pub zip: String,
    pub country: String,
    pub units: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponseWasm {
    pub location: Location,
    pub weather: String,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub async fn get_weather_data(request_json: &str) -> Result<String, JsValue> {
    console_log!("WASM function called with: {}", request_json);

    let request: WeatherRequestWasm = serde_json::from_str(request_json)
        .map_err(|e| {
            console_log!("JSON parse error: {}", e);
            JsValue::from_str(&format!("Invalid request: {}", e))
        })?;

    console_log!("Parsed request: {:?}", request);

    match fetch_weather_internal(request).await {
        Ok(response) => {
            console_log!("Weather fetch successful");
            serde_json::to_string(&response)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(e) => {
            console_log!("Weather fetch error: {}", e);
            let error_response = WeatherResponseWasm {
                location: Location {
                    zip: String::new(),
                    name: String::new(),
                    lat: 0.0,
                    lon: 0.0,
                    country: String::new(),
                },
                weather: String::new(),
                error: Some(e),
            };
            serde_json::to_string(&error_response)
                .map_err(|e| JsValue::from_str(&format!("Error serialization failed: {}", e)))
        }
    }
}

async fn fetch_weather_internal(request: WeatherRequestWasm) -> Result<WeatherResponseWasm, String> {
    console_log!("Creating location client");
    console_log!("Fetching location");

    let location = LocationClient::new(
        request.zip.clone(),
        request.country.clone(),
        request.api_key.clone(), )
        .get_location()
        .await
        .map_err(|e| format!("Location error: {}", e))?;

    console_log!("Location found: {:?}", location);
    console_log!("Fetching weather");

    let weather_response = WeatherClient::new(
        location.clone(),
        request.units.clone(),
        request.api_key.clone(), )
        .get_current_weather()
        .await
        .map_err(|e| format!("Weather error: {}", e))?;

    console_log!("Weather fetch complete");

    Ok(WeatherResponseWasm {
        location,
        weather: serde_json::to_string(&weather_response)
            .map_err(|e| format!("Weather serialization error: {}", e))?,
        error: None,
    })
}