use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::location::{Location, LocationClient};
use crate::weather::WeatherClient;

// region: Coord

/// Represents geographic coordinates with longitude and latitude in decimal degrees.
///
/// Provides validation to ensure longitude is between -180 and 180 degrees,
/// and latitude is between -90 and 90 degrees.
///
/// Sample JSON
/// ```json
/// "coord": {
///     "lon": -82.1993,
///     "lat": 42.4421
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Latitude in decimal degrees
    pub lat: f64,
}

impl Coord {
    pub fn new(lon: f64, lat: f64) -> Result<Self, String> {
        if lon < -180.0 || lon > 180.0 {
            return Err("Longitude must be between -180 and 180 degrees".to_string());
        }
        if lat < -90.0 || lat > 90.0 {
            return Err("Latitude must be between -90 and 90 degrees".to_string());
        }
        Ok(Coord { lon, lat })
    }
}

// endregion

// region Weather

/// Represents a weather condition as provided by the OpenWeatherMap API.
///
/// Contains an internal weather condition ID, the main group of weather parameters
/// (such as Rain, Snow, Clouds), a detailed description of the weather condition,
/// and an icon ID used for displaying corresponding weather icons.
///
/// Sample JSON
/// ```json
/// "weather": [
///     {
///         "id": 801,
///         "main": "Clouds",
///         "description": "few clouds",
///         "icon": "02d"
///     }
/// ]
/// ```
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

// endregion

// region: Main

/// Represents the main weather parameters including temperature, pressure, and humidity.
///
/// All fields are optional to accommodate partial data scenarios commonly returned
/// by weather APIs. Temperatures are typically in Kelvin by default, but may also be
/// provided in Celsius or Fahrenheit depending on the API settings. Pressure values
/// are given in hPa, and humidity is expressed as a percentage.
///
/// # Fields
/// - `temp`: Current temperature.
/// - `feels_like`: Human-perceived temperature.
/// - `temp_min`: Minimum observed temperature at the moment (mostly for large cities).
/// - `temp_max`: Maximum observed temperature at the moment (mostly for large cities).
/// - `pressure`: Atmospheric pressure on the sea level in hPa.
/// - `humidity`: Humidity in percentage.
/// - `sea_level`: Atmospheric pressure on the sea level in hPa (optional).
/// - `grnd_level`: Atmospheric pressure on the ground level in hPa (optional).
///
/// Sample JSON
/// ```json
/// "main": {
///     "temp": 27.77,
///     "feels_like": 29.32,
///     "temp_min": 27.77,
///     "temp_max": 29.1,
///     "pressure": 1014,
///     "humidity": 62,
///     "sea_level": 1014,
///     "grnd_level": 993
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Main {
    /// Current temperature
    pub temp: Option<f64>,
    /// Temperature perception by humans
    pub feels_like: Option<f64>,
    /// Minimum temperature at the moment (for large cities and urban areas)
    pub temp_min: Option<f64>,
    /// Maximum temperature at the moment (for large cities and urban areas)
    pub temp_max: Option<f64>,
    /// Atmospheric pressure on the sea level in hPa
    pub pressure: Option<i32>,
    /// Humidity percentage
    pub humidity: Option<i32>,
    /// Atmospheric pressure on the sea level in hPa (optional)
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level in hPa (optional)
    pub grnd_level: Option<i32>,
}

impl Main {
    pub fn new(temp: Option<f64>, feels_like: Option<f64>, temp_min: Option<f64>, temp_max: Option<f64>, pressure: Option<i32>, humidity: Option<i32>, sea_level: Option<i32>, grnd_level: Option<i32>) -> Result<Self, String> {
        if let Some(humidity_value) = humidity {
            if humidity_value < 0 || humidity_value > 100 {
                return Err("Humidity must be between 0 and 100 percent".to_string());
            }
        }

        Ok(Main { temp, feels_like, temp_min, temp_max, pressure, humidity, sea_level, grnd_level })
    }
}

// endregion

// region: Wind

/// Represents meteorological wind data including speed, direction, and gusts.
///
/// - `speed`: Wind speed with units depending on the API request (e.g., meters per second for metric or miles per hour for imperial).
/// - `deg`: Wind direction in degrees, measured meteorologically.
/// - `gust`: Optional wind gust speed, same units as `speed`.
///
/// Sample JSON
/// ```json
/// "wind": {
///     "speed": 3.6,
///     "deg": 220
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wind {
    /// Wind speed (units vary by API request: m/s for metric, mph for imperial)
    pub speed: f64,
    /// Wind direction in degrees (meteorological)
    pub deg: i32,
    /// Wind gust speed (optional, same units as speed)
    pub gust: Option<f64>,
}

impl Wind {
    pub fn new(speed: f64, deg: i32, gust: Option<f64>) -> Result<Self, String> {
        // Validate speed minimum
        if speed < 0.0 {
            return Err("Wind speed must be non-negative".to_string());
        }

        // Validate degree range - FIXED to match schema
        if deg < 0 || deg > 360 {
            return Err("Wind direction must be between 0 and 360 degrees (inclusive)".to_string());
        }

        // Validate gust minimum if present
        if let Some(gust_value) = gust {
            if gust_value < 0.0 {
                return Err("Wind gust must be non-negative".to_string());
            }
        }

        Ok(Wind { speed, deg, gust })
    }
}

// endregion

// region: Clouds

/// Represents cloudiness information as a percentage.
///
/// The `Clouds` struct contains a single field `all` which indicates the
/// overall cloudiness in the sky, expressed as a percentage value from 0 to 100.
///
/// Sample JSON
/// ```json
/// "clouds": {
///     "all": 20
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clouds {
    /// Cloudiness, %
    pub all: i32,
}

impl Clouds {
    pub fn new(all: i32) -> Result<Self, String> {
        if all < 0 || all > 100 {
            return Err("Clouds::all must be between 0 and 100 percent".to_string());
        }

        Ok(Clouds { all })
    }
}

// endregion

// region Sys

/// Represents system-related metadata typically returned by the weather API.
///
/// This struct contains internal parameters such as type and id, as well as
/// geographical and astronomical data like the country code and times for sunrise and sunset.
///
/// Fields:
/// - `sys_type`: Internal parameter indicating the type.
/// - `id`: Internal identifier.
/// - `country`: ISO 3166-1 alpha-2 country code.
/// - `sunrise`: Sunrise time as a Unix timestamp (UTC).
/// - `sunset`: Sunset time as a Unix timestamp (UTC).
///
/// Sample JSON
/// ```json
/// "sys": {
///     "type": 2,
///     "id": 267607,
///     "country": "CA",
///     "sunrise": 1752401008,
///     "sunset": 1752455116
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sys {
    /// Internal parameter
    #[serde(rename = "type")]
    pub sys_type: i32,
    /// Internal parameter
    pub id: i32,
    /// Country code (ISO 3166-1 alpha-2)
    pub country: String,
    /// Sunrise time, unix, UTC
    pub sunrise: i64,
    /// Sunset time, unix, UTC
    pub sunset: i64,
}

impl Sys {
    pub fn new(sys_type: i32, id: i32, country: String, sunrise: i64, sunset: i64) -> Result<Self, String> {
        // Validate country code format (ISO 3166-1 alpha-2)
        if !country.chars().all(|c| c.is_ascii_uppercase()) || country.len() != 2 {
            return Err("Country code must be exactly 2 uppercase letters (ISO 3166-1 alpha-2)".to_string());
        }

        Ok(Sys {
            sys_type,
            id,
            country,
            sunrise,
            sunset,
        })
    }
}

// endregion

// region WeatherResponse

/// Represents the full weather data response from the weather API.
///
/// This struct contains detailed information about the weather at a specific location,
/// including geographic coordinates, weather conditions, temperature, wind, clouds,
/// visibility, time of data calculation, system information, timezone offset,
/// and location identifiers.
///
/// Fields:
/// - `coord`: Geographic coordinates of the location.
/// - `weather`: List of weather condition descriptors (e.g., rain, clear sky).
/// - `base`: Internal parameter indicating data source.
/// - `main`: Main weather measurements such as temperature, pressure, and humidity.
/// - `visibility`: Visibility distance in meters.
/// - `wind`: Wind speed and direction data.
/// - `clouds`: Cloud coverage data.
/// - `dt`: Timestamp of the weather data calculation in Unix UTC time.
/// - `sys`: System-related information including country code, sunrise, and sunset times.
/// - `timezone`: Offset in seconds from UTC.
/// - `id`: Unique city or location identifier.
/// - `name`: Name of the city or location.
/// - `cod`: Internal parameter, usually representing the status code of the API response.
///
/// Sample JSON
/// ```json
/// {
///     "coord": {
///         "lon": -82.1993,
///         "lat": 42.4421
///     },
///     "weather": [
///         {
///             "id": 801,
///             "main": "Clouds",
///             "description": "few clouds",
///             "icon": "02d"
///         }
///     ],
///     "base": "stations",
///     "main": {
///         "temp": 27.77,
///         "feels_like": 29.32,
///         "temp_min": 27.77,
///         "temp_max": 29.1,
///         "pressure": 1014,
///         "humidity": 62,
///         "sea_level": 1014,
///         "grnd_level": 993
///     },
///     "visibility": 10000,
///     "wind": {
///         "speed": 3.6,
///         "deg": 220
///     },
///     "clouds": {
///         "all": 20
///     },
///     "dt": 1752449935,
///     "sys": {
///         "type": 2,
///         "id": 267607,
///         "country": "CA",
///         "sunrise": 1752401008,
///         "sunset": 1752455116
///     },
///     "timezone": -14400,
///     "id": 5920450,
///     "name": "Chatham-Kent",
///     "cod": 200
/// }
/// ```
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
    pub visibility: i64,
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
    pub cod: i64,
}

impl WeatherResponse {
    pub fn new(
        coord: Coord,
        weather: Vec<Weather>,
        base: String,
        main: Main,
        visibility: i64,
        wind: Wind,
        clouds: Clouds,
        dt: i64,
        sys: Sys,
        timezone: i32,
        id: i64,
        name: String,
        cod: i64) -> Result<Self, String> {
        // Validate Visibility
        if visibility < 0 {
            return Err("Visibility must never be less than 0.".to_string());
        }

        Ok(WeatherResponse { coord, weather, base, main, visibility, wind, clouds, dt, sys, timezone, id, name, cod })
    }

    pub fn detailed_display(&self, units: String) -> String {
        // Temperature
        let mut temp_display : String = "".to_string();
        if let Some(temp_value) = &self.main.temp {
            temp_display = get_temperature_display(temp_value, &units);
        }

        // Wind
        let wind_display = get_speed_display(self.wind.speed, &units);

        // Weather
        let mut weather_main = "";
        let mut weather_description = "";
        let mut weather_icon = "";
        if let Some(weather) = self.weather.first() {
            weather_main = &weather.main;
            weather_description = &weather.description;
            weather_icon = &weather.icon;
        }

        format!(
            r#"🌤️ Weather in {}
📍 Coordinates: ({}, {})
🌡️ Temperature: {}
💨 Wind: {} at {}°
☁️ Clouds: {}%
🌈 Conditions: {} ({})
   Icon: {}"#,
            self.name,
            self.coord.lat,
            self.coord.lon,
            temp_display,
            wind_display,
            self.wind.deg,
            self.clouds.all,
            weather_main,
            weather_description,
            weather_icon,
        )
    }
}

// endregion

fn get_temperature_display(temp: &f64, units: &str) -> String {
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