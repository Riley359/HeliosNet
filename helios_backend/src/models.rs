use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    pub id: i32,
    pub name: String,
    pub data_source: String,
    pub location: Option<String>, // Will store as WKT (Well-Known Text) for easier handling
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorLocation {
    pub id: i32,
    pub name: String,
    pub data_source: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct NewSensor {
    pub name: String,
    pub data_source: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<Sensor> for SensorLocation {
    fn from(sensor: Sensor) -> Self {
        // Parse the WKT point format "POINT(longitude latitude)"
        let (longitude, latitude) = if let Some(location_str) = &sensor.location {
            let coords = location_str
                .trim_start_matches("POINT(")
                .trim_end_matches(")")
                .split_whitespace()
                .map(|s| s.parse::<f64>().unwrap_or(0.0))
                .collect::<Vec<f64>>();
            
            if coords.len() >= 2 {
                (coords[0], coords[1])
            } else {
                (0.0, 0.0)
            }
        } else {
            (0.0, 0.0)
        };

        SensorLocation {
            id: sensor.id,
            name: sensor.name,
            data_source: sensor.data_source,
            latitude,
            longitude,
        }
    }
}
