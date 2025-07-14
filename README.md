# OpenWeatherMap Rust Library

A Rust library for interacting with the OpenWeatherMap API. This library supports both native Rust applications and WebAssembly targets.

## Features

- Current weather data retrieval
- Location-based weather lookups
- Fully typed API responses
- WebAssembly compatibility

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openweathermap_lib = "0.1.0-pre.2"
```

## Usage

```rust
use openweathermap_lib::weather::client::WeatherClient;
use openweathermap_lib::location::types::Location;

async fn example() {
    // Initialize with your API key
    let client = WeatherClient::new("your_api_key");

    // Get weather by location
    let location = Location {
        latitude: 40.7128,
        longitude: -74.0060,
    };

    let weather = client.get_current_weather(location).await.unwrap();
    println!("Temperature: {}°C", weather.main.temp);
}
```

## Project Structure

```
.
├── src/                  # Source code
│   ├── lib.rs            # Library entry point
│   ├── weather/          # Weather-related functionality
│   └── location/         # Location-related functionality
├── schemas/              # JSON schemas
├── Cargo.toml            # Rust package manifest
├── Cargo.lock            # Dependency lock file
├── LICENSE               # MIT License
└── CONTRIBUTING.md       # Contribution guidelines
```

## Cloning the Repository

To clone this repository, run the following command:

```bash
git clone https://github.com/tonymarkham/rusty-openweathermap-library.git
cd rusty-openweathermap-library
```

## Building

### Native Rust

```bash
cargo build --release
```

### WebAssembly

Ensure you have wasm-pack installed:

```bash
cargo install wasm-pack
wasm-pack build --target web
```

## Dependencies

- reqwest 0.12.22
- serde 1.0.219
- serde_json 1.0.140
- wasm-bindgen 0.2.100
- wasm-bindgen-futures 0.4.50

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## Author

- Tony Markham
