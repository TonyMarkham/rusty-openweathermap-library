use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    /// ZIP or postal code
    pub zip: String,
    /// City or locality name
    pub name: String,
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Two-letter country code (ISO 3166-1 alpha-2)
    pub country: String,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "name: [{}]\ncountry: [{}]\nzip: [{}]\nlat: [{}]\nlon: [{}]",
               self.name, self.country, self.zip, self.lat, self.lon)
    }
}
