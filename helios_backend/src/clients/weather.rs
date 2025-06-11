use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub main: Main,
    pub wind: Wind,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: f64,
    pub humidity: u8,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
}

// Add this struct for easier access to weather data
#[derive(Debug)]
pub struct WeatherData {
    pub temperature: f64,
    pub humidity: u8,
    pub wind_speed: f64,
    pub wind_direction: f64,
}

impl From<WeatherResponse> for WeatherData {
    fn from(response: WeatherResponse) -> Self {
        WeatherData {
            temperature: response.main.temp,
            humidity: response.main.humidity,
            wind_speed: response.wind.speed,
            wind_direction: response.wind.deg,
        }
    }
}

pub async fn get_weather_by_coords(lat: f64, lon: f64) -> Result<WeatherData, String> {
    let api_key = std::env::var("WEATHER_API_KEY").map_err(|_| "WEATHER_API_KEY must be set".to_string())?;
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric", lat, lon, api_key);
    
    let response = reqwest::get(&url).await
        .map_err(|e| format!("Weather API request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Weather API request failed with status: {}", response.status()));
    }
    
    let weather_response: WeatherResponse = response.json().await
        .map_err(|e| format!("Failed to parse weather response: {}", e))?;
    
    Ok(WeatherData::from(weather_response))
}