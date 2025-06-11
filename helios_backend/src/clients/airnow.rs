// src/clients/airnow.rs

use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct AirNowResponse {
    #[serde(rename = "AQI")]
    pub aqi: i32,
    #[serde(rename = "Category")]
    pub category: CategoryInfo,
    #[serde(rename = "DateObserved")]
    #[allow(dead_code)]
    pub date_observed: String,
    #[serde(rename = "HourObserved")]
    #[allow(dead_code)]
    pub hour_observed: i32,
    #[serde(rename = "LocalTimeZone")]
    #[allow(dead_code)]
    pub local_time_zone: String,
    #[serde(rename = "ReportingArea")]
    #[allow(dead_code)]
    pub reporting_area: String,
    #[serde(rename = "StateCode")]
    #[allow(dead_code)]
    pub state_code: String,
    #[serde(rename = "Latitude")]
    #[allow(dead_code)]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    #[allow(dead_code)]
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
pub struct CategoryInfo {
    #[serde(rename = "Number")]
    #[allow(dead_code)]
    pub number: i32,
    #[serde(rename = "Name")]
    pub name: String,
}

// Simple struct for the main.rs handler
#[derive(Debug)]
pub struct AirQualityIndex {
    pub aqi: i32,
    pub category: String,
}

impl From<AirNowResponse> for AirQualityIndex {
    fn from(response: AirNowResponse) -> Self {
        AirQualityIndex {
            aqi: response.aqi,
            category: response.category.name,
        }
    }
}

pub async fn get_aqi_by_zip(zip_code: &str) -> Result<AirQualityIndex, String> {
    let api_key = env::var("AIRNOW_API_KEY").map_err(|_| "AIRNOW_API_KEY not set".to_string())?;
    let url = format!(
        "https://www.airnowapi.org/aq/observation/zipCode/current/?format=application/json&zipCode={}&distance=25&API_KEY={}", 
        zip_code, api_key
    );
    
    let client = Client::new();
    let response = client.get(&url).send().await
        .map_err(|e| format!("API request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()));
    }
    
    let air_data: Vec<AirNowResponse> = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Find the PM2.5 or Ozone reading (most common AQI measurements)
    air_data
        .into_iter()
        .next()
        .map(AirQualityIndex::from)
        .ok_or_else(|| "No air quality data found for this zip code".to_string())
}