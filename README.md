# openweathermap_lib

A Rust library for interacting with the OpenWeatherMap API, supporting both native and WebAssembly targets.

## Features

- Current weather data retrieval
- Location-based weather lookups
- Fully typed API responses
- WebAssembly support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openweathermap_lib = "0.1.0"
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

## WebAssembly Usage

This library also supports WebAssembly for use in browser environments.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.