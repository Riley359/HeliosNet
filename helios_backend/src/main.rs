use axum::{
    routing::get,
    Router,
    Json,
    extract::{Path, Query, State},
    http::Method,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;

mod config;
mod clients;
mod models;
mod database;
mod ml;

use database::Database;
use ml::{RiskModel, WeatherData};

#[derive(Deserialize)]
struct LocationQuery {
    lat: Option<f64>,
    lon: Option<f64>,
}

#[derive(Deserialize)]
struct BoundsQuery {
    min_lat: Option<f64>,
    min_lon: Option<f64>,
    max_lat: Option<f64>,
    max_lon: Option<f64>,
    lat: Option<f64>,
    lon: Option<f64>,
}

#[derive(Deserialize)]
struct RiskQuery {
    lat: f64,
    lon: f64,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
    risk_model: Arc<RiskModel>,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct EnvironmentalResponse {
    air_quality: AirQualityResponse,
    weather: WeatherResponse,
    location: LocationResponse,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct AirQualityResponse {
    aqi: i32,
    category: String,
    location: String,
    timestamp: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct WeatherResponse {
    temperature: f64,
    humidity: f64,
    wind_speed: f64,
    wind_direction: f64,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct LocationResponse {
    latitude: f64,
    longitude: f64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Set up database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    
    // Run pending migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    let db = Arc::new(Database::new(pool));
    
    // Initialize the ML risk model
    let risk_model = Arc::new(RiskModel::new().expect("Failed to load risk model"));
    
    let app_state = AppState { db, risk_model };
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);
    
    let app = Router::new()
        .route("/environmental-data", get(environmental_data_handler))
        .route("/health", get(health_handler))
        .route("/api/status/:zipcode", get(status_handler)) // Keep old endpoint for compatibility
        .route("/api/sensors", get(sensors_handler))
        .route("/api/risk/point", get(risk_prediction_handler))
        .with_state(app_state)
        .layer(ServiceBuilder::new().layer(cors));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on http://127.0.0.1:8080");
    println!("API endpoints:");
    println!("  GET /health - Health check");
    println!("  GET /environmental-data?lat=44.1&lon=-121.7 - Environmental data");
    println!("  GET /api/sensors?min_lat=44&min_lon=-122&max_lat=45&max_lon=-121 - Sensors in bounds");
    println!("  GET /api/status/:zipcode - Legacy status endpoint");
    println!("  GET /api/risk/point?lat=44.1&lon=-121.7 - Fire risk prediction");
    
    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn environmental_data_handler(
    State(state): State<AppState>,
    Query(params): Query<LocationQuery>
) -> Json<serde_json::Value> {
    // Use provided coordinates or default to Altamont, Oregon
    let (lat, lon) = (
        params.lat.unwrap_or(44.1292),
        params.lon.unwrap_or(-121.7689)
    );
    
    // For now, use zip code 97601 (Klamath Falls, OR) as a proxy for the region
    let zip_code = "97601";

    let air_quality_future = clients::get_aqi_by_zip(zip_code);
    let weather_future = clients::get_weather_by_coords(lat, lon);
    let sensors_future = state.db.get_sensors_near_point(lat, lon, 25.0);

    let (air_quality, weather, sensors) = tokio::join!(air_quality_future, weather_future, sensors_future);

    let nearby_sensors = sensors.unwrap_or_default();

    match (air_quality, weather) {
        (Ok(aqi_data), Ok(weather_data)) => {
            Json(json!({
                "air_quality": {
                    "aqi": aqi_data.aqi,
                    "category": aqi_data.category,
                    "location": format!("Altamont, Oregon"),
                    "timestamp": chrono::Utc::now().to_rfc3339()
                },
                "weather": {
                    "temperature": weather_data.temperature,
                    "humidity": weather_data.humidity,
                    "wind_speed": weather_data.wind_speed,
                    "wind_direction": weather_data.wind_direction
                },
                "location": {
                    "latitude": lat,
                    "longitude": lon
                },
                "sensors": nearby_sensors
            }))
        }
        (Err(aqi_err), Ok(weather_data)) => {
            Json(json!({
                "air_quality": {
                    "aqi": 0,
                    "category": "Data Unavailable",
                    "location": "Altamont, Oregon",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                },
                "weather": {
                    "temperature": weather_data.temperature,
                    "humidity": weather_data.humidity,
                    "wind_speed": weather_data.wind_speed,
                    "wind_direction": weather_data.wind_direction
                },
                "location": {
                    "latitude": lat,
                    "longitude": lon
                },
                "sensors": nearby_sensors,
                "error": format!("Failed to fetch air quality data: {}", aqi_err)
            }))
        }
        (Ok(aqi_data), Err(weather_err)) => {
            Json(json!({
                "air_quality": {
                    "aqi": aqi_data.aqi,
                    "category": aqi_data.category,
                    "location": "Altamont, Oregon",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                },
                "weather": {
                    "temperature": 0.0,
                    "humidity": 0.0,
                    "wind_speed": 0.0,
                    "wind_direction": 0.0
                },
                "location": {
                    "latitude": lat,
                    "longitude": lon
                },
                "sensors": nearby_sensors,
                "error": format!("Failed to fetch weather data: {}", weather_err)
            }))
        }
        (Err(aqi_err), Err(weather_err)) => {
            Json(json!({
                "air_quality": {
                    "aqi": 0,
                    "category": "Data Unavailable",
                    "location": "Altamont, Oregon",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                },
                "weather": {
                    "temperature": 0.0,
                    "humidity": 0.0,
                    "wind_speed": 0.0,
                    "wind_direction": 0.0
                },
                "location": {
                    "latitude": lat,
                    "longitude": lon
                },
                "sensors": nearby_sensors,
                "error": format!("Failed to fetch data - AQI: {}, Weather: {}", aqi_err, weather_err)
            }))
        }
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[axum::debug_handler]
async fn status_handler(Path(zip_code): Path<String>) -> Json<serde_json::Value> {
    let (lat, lon) = (42.19, -121.78); // Hardcoded coordinates for Altamont

    let air_quality_future = clients::get_aqi_by_zip(&zip_code);
    let weather_future = clients::get_weather_by_coords(lat, lon);

    let (air_quality, weather) = tokio::join!(air_quality_future, weather_future);

    match (air_quality, weather) {
        (Ok(aqi_data), Ok(weather_data)) => {
            Json(json!({
                "aqi": aqi_data.aqi,
                "aqi_category": aqi_data.category,
                "temperature": weather_data.temperature,
                "humidity": weather_data.humidity,
                "wind_speed": weather_data.wind_speed,
                "wind_direction": weather_data.wind_direction
            }))
        }
        (Err(aqi_err), Ok(weather_data)) => {
            Json(json!({
                "error": format!("Failed to fetch air quality data: {}", aqi_err),
                "temperature": weather_data.temperature,
                "humidity": weather_data.humidity,
                "wind_speed": weather_data.wind_speed,
                "wind_direction": weather_data.wind_direction
            }))
        }
        (Ok(aqi_data), Err(weather_err)) => {
            Json(json!({
                "aqi": aqi_data.aqi,
                "aqi_category": aqi_data.category,
                "error": format!("Failed to fetch weather data: {}", weather_err)
            }))
        }
        (Err(aqi_err), Err(weather_err)) => {
            Json(json!({
                "error": format!("Failed to fetch data - AQI: {}, Weather: {}", aqi_err, weather_err)
            }))
        }
    }
}

#[axum::debug_handler]
async fn sensors_handler(
    State(state): State<AppState>,
    Query(params): Query<BoundsQuery>
) -> Json<serde_json::Value> {
    // If bounds are provided, use them; otherwise get all sensors
    let sensors_result = if let (Some(min_lat), Some(min_lon), Some(max_lat), Some(max_lon)) = 
        (params.min_lat, params.min_lon, params.max_lat, params.max_lon) {
        state.db.get_sensors_in_bounds(min_lat, min_lon, max_lat, max_lon).await
    } else if let (Some(lat), Some(lon)) = (params.lat, params.lon) {
        // Get sensors within 50km of the point
        state.db.get_sensors_near_point(lat, lon, 50.0).await
    } else {
        state.db.get_all_sensors().await
    };

    match sensors_result {
        Ok(sensors) => {
            Json(json!({
                "sensors": sensors,
                "count": sensors.len(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
        Err(e) => {
            Json(json!({
                "error": format!("Failed to fetch sensors: {}", e),
                "sensors": [],
                "count": 0,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
    }
}

#[axum::debug_handler]
async fn risk_prediction_handler(
    State(state): State<AppState>,
    Query(params): Query<RiskQuery>
) -> Json<serde_json::Value> {
    let lat = params.lat;
    let lon = params.lon;
    
    // Fetch current weather data for the location
    match clients::get_weather_by_coords(lat, lon).await {
        Ok(weather_data) => {
            // Convert weather data to ML model format
            let ml_weather_data = WeatherData {
                temperature: weather_data.temperature as f32,
                humidity: weather_data.humidity as f32,
                wind_speed: weather_data.wind_speed as f32,
                precipitation: 0.0, // Default - could be enhanced with historical data
                drought_index: calculate_drought_index(&weather_data), // Calculated from current conditions
            };
            
            // Make risk prediction
            match state.risk_model.predict(&ml_weather_data) {
                Ok(risk_probability) => {
                    let risk_level = match risk_probability {
                        p if p >= 0.8 => "EXTREME",
                        p if p >= 0.6 => "HIGH", 
                        p if p >= 0.4 => "MODERATE",
                        p if p >= 0.2 => "LOW",
                        _ => "MINIMAL"
                    };
                    
                    Json(json!({
                        "location": {
                            "latitude": lat,
                            "longitude": lon
                        },
                        "risk": {
                            "probability": risk_probability,
                            "level": risk_level,
                            "description": get_risk_description(risk_level)
                        },
                        "weather_conditions": {
                            "temperature": weather_data.temperature,
                            "humidity": weather_data.humidity,
                            "wind_speed": weather_data.wind_speed,
                            "wind_direction": weather_data.wind_direction
                        },
                        "model_inputs": {
                            "temperature": ml_weather_data.temperature,
                            "humidity": ml_weather_data.humidity,
                            "wind_speed": ml_weather_data.wind_speed,
                            "precipitation": ml_weather_data.precipitation,
                            "drought_index": ml_weather_data.drought_index
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }))
                }
                Err(ml_err) => {
                    Json(json!({
                        "error": format!("Failed to make risk prediction: {}", ml_err),
                        "location": {
                            "latitude": lat,
                            "longitude": lon
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }))
                }
            }
        }
        Err(weather_err) => {
            Json(json!({
                "error": format!("Failed to fetch weather data: {}", weather_err),
                "location": {
                    "latitude": lat,
                    "longitude": lon
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }
    }
}

fn calculate_drought_index(weather_data: &clients::weather::WeatherData) -> f32 {
    // Simple drought index calculation based on temperature and humidity
    // Higher temperature + lower humidity = higher drought index
    let temp_factor = (weather_data.temperature - 32.0) / 100.0; // Normalize from Fahrenheit
    let humidity_factor = 1.0 - (weather_data.humidity as f64 / 100.0);
    let drought_index = ((temp_factor + humidity_factor) * 50.0).max(0.0).min(100.0);
    drought_index as f32
}

fn get_risk_description(level: &str) -> String {
    match level {
        "EXTREME" => "Extreme fire danger. Avoid all outdoor burning and activities that could spark fires.".to_string(),
        "HIGH" => "High fire danger. Exercise extreme caution with any potential ignition sources.".to_string(),
        "MODERATE" => "Moderate fire danger. Use caution with outdoor activities and burning.".to_string(),
        "LOW" => "Low fire danger. Normal fire safety precautions apply.".to_string(),
        "MINIMAL" => "Minimal fire danger. Conditions are favorable for fire safety.".to_string(),
        _ => "Fire risk assessment unavailable.".to_string()
    }
}