# Helios Backend

Helios Backend is a Rust application that aggregates live environmental data from external APIs and serves it through a simple web interface. This project utilizes the Axum framework for building the API and integrates with AirNow and a weather API to provide real-time air quality and weather information.

## Project Structure

```
helios_backend
├── src
│   ├── main.rs          # Entry point of the application, sets up the API
│   ├── config.rs        # Loads environment variables into a strongly-typed struct
│   └── clients
│       ├── mod.rs       # Central module for API clients
│       ├── airnow.rs    # Client for fetching air quality data from AirNow API
│       └── weather.rs    # Client for fetching weather data from a weather API
├── Cargo.toml           # Project dependencies and metadata
├── .env                 # Environment variables and API keys
└── README.md            # Project documentation
```

## Setup Instructions

1. **Clone the repository:**
   ```
   git clone https://github.com/microsoft/vscode-remote-try-rust.git
   cd helios_backend
   ```

2. **Install Rust:**
   Ensure you have Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).

3. **Configure Environment Variables:**
   Create a `.env` file in the project root and add your API keys:
   ```
   AIRNOW_API_KEY=your_airnow_api_key
   WEATHER_API_KEY=your_weather_api_key
   ```

4. **Add Dependencies:**
   The necessary dependencies are already specified in `Cargo.toml`. You can install them by running:
   ```
   cargo build
   ```

5. **Run the Application:**
   Start the server with:
   ```
   cargo run
   ```

## Usage

Once the server is running, you can access the API endpoint to get the environmental data:

```
GET /api/status/:zipcode
```

Replace `:zipcode` with the desired zip code to retrieve the air quality index and weather information.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.