use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)]
    pub airnow_api_key: String,
    #[allow(dead_code)]
    pub weather_api_key: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        
        let airnow_api_key = env::var("AIRNOW_API_KEY")?;
        let weather_api_key = env::var("WEATHER_API_KEY")?;
        
        Ok(Config {
            airnow_api_key,
            weather_api_key,
        })
    }
}