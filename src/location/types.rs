use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a geographical location identified by postal code.
///
/// Contains essential information about the location including the postal code (`zip`),
/// the name of the city or locality, its latitude and longitude coordinates,
/// and the associated country represented by its ISO 3166-1 alpha-2 country code.
///
/// Sample JSON
/// ```json
/// {
///     "zip": "N7L",
///     "name": "Chatham",
///     "lat": 42.4209,
///     "lon": -82.1993,
///     "country": "CA"
/// }
/// ```
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
        write!(
            f,
            "name: [{}]\ncountry: [{}]\nzip: [{}]\nlat: [{}]\nlon: [{}]",
            self.name, self.country, self.zip, self.lat, self.lon
        )
    }
}
