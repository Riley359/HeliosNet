use std::error::Error;
use serde::Deserialize;
use dotenvy::dotenv;

mod models {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Deserialize)]
    pub struct NewSensor {
        pub name: String,
        pub data_source: String,
        pub latitude: f64,
        pub longitude: f64,
    }
}

mod database {
    use sqlx::{Pool, Postgres, Error as SqlxError};
    use crate::models::NewSensor;
    
    pub struct Database {
        pool: Pool<Postgres>,
    }
    
    impl Database {
        pub fn new(pool: Pool<Postgres>) -> Self {
            Self { pool }
        }
        
        pub async fn insert_sensor(&self, sensor: NewSensor) -> Result<i32, SqlxError> {
            let record = sqlx::query!(
                r#"
                INSERT INTO sensors (name, data_source, location)
                VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326))
                RETURNING id
                "#,
                sensor.name,
                sensor.data_source,
                sensor.longitude,
                sensor.latitude
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(record.id)
        }
        
        pub async fn get_sensor_count(&self) -> Result<i64, SqlxError> {
            let record = sqlx::query!("SELECT COUNT(*) as count FROM sensors")
                .fetch_one(&self.pool)
                .await?;
            
            Ok(record.count.unwrap_or(0))
        }
    }
}

use models::NewSensor;
use database::Database;

#[derive(Debug, Deserialize)]
struct AirNowSensor {
    #[serde(rename = "Site Name")]
    site_name: String,
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(rename = "State")]
    state: Option<String>,
    #[serde(rename = "City")]
    city: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    let db = Database::new(pool);
    
    // Sample AirNow sensor data for Oregon region
    // In a real implementation, you would download this from:
    // https://www.airnow.gov/index.cfm?action=airnow.global_summary
    let sample_sensors = vec![
        AirNowSensor {
            site_name: "Portland-SE Lafayette".to_string(),
            latitude: 45.4871,
            longitude: -122.6037,
            state: Some("Oregon".to_string()),
            city: Some("Portland".to_string()),
        },
        AirNowSensor {
            site_name: "Eugene-Amazon Park".to_string(),
            latitude: 44.0462,
            longitude: -123.0351,
            state: Some("Oregon".to_string()),
            city: Some("Eugene".to_string()),
        },
        AirNowSensor {
            site_name: "Medford-Welch Street".to_string(),
            latitude: 42.3265,
            longitude: -122.8756,
            state: Some("Oregon".to_string()),
            city: Some("Medford".to_string()),
        },
        AirNowSensor {
            site_name: "Bend-NE 27th Street".to_string(),
            latitude: 44.0582,
            longitude: -121.2767,
            state: Some("Oregon".to_string()),
            city: Some("Bend".to_string()),
        },
        AirNowSensor {
            site_name: "Salem-Lancaster Drive".to_string(),
            latitude: 44.9429,
            longitude: -123.0351,
            state: Some("Oregon".to_string()),
            city: Some("Salem".to_string()),
        },
        AirNowSensor {
            site_name: "Klamath Falls-Lakeview".to_string(),
            latitude: 42.2249,
            longitude: -121.7817,
            state: Some("Oregon".to_string()),
            city: Some("Klamath Falls".to_string()),
        },
        AirNowSensor {
            site_name: "Corvallis-Circle Boulevard".to_string(),
            latitude: 44.5646,
            longitude: -123.2620,
            state: Some("Oregon".to_string()),
            city: Some("Corvallis".to_string()),
        },
    ];
    
    println!("Seeding {} sensors into database...", sample_sensors.len());
    
    for sensor_data in sample_sensors {
        let full_name = if let (Some(city), Some(state)) = (&sensor_data.city, &sensor_data.state) {
            format!("{} - {}, {}", sensor_data.site_name, city, state)
        } else {
            sensor_data.site_name.clone()
        };
        
        let new_sensor = NewSensor {
            name: full_name,
            data_source: "AirNow".to_string(),
            latitude: sensor_data.latitude,
            longitude: sensor_data.longitude,
        };
        
        match db.insert_sensor(new_sensor).await {
            Ok(id) => println!("✓ Inserted sensor '{}' with ID {}", sensor_data.site_name, id),
            Err(e) => eprintln!("✗ Failed to insert sensor '{}': {}", sensor_data.site_name, e),
        }
    }
    
    println!("Sensor seeding completed!");
      // Verify the data was inserted
    match db.get_sensor_count().await {
        Ok(count) => {
            println!("\nTotal sensors in database: {}", count);
        }
        Err(e) => eprintln!("Failed to verify sensors: {}", e),
    }
    
    Ok(())
}
