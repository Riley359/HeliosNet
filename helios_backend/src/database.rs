use sqlx::{Pool, Postgres, Error as SqlxError, Row};
use crate::models::{Sensor, SensorLocation, NewSensor};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_sensors_in_bounds(
        &self,
        min_lat: f64,
        min_lon: f64,
        max_lat: f64,
        max_lon: f64,
    ) -> Result<Vec<SensorLocation>, SqlxError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                data_source,
                ST_AsText(location) as location,
                created_at,
                updated_at
            FROM sensors 
            WHERE ST_Intersects(
                location, 
                ST_MakeEnvelope($1, $2, $3, $4, 4326)
            )
            ORDER BY name
            "#
        )
        .bind(min_lon)
        .bind(min_lat)
        .bind(max_lon)
        .bind(max_lat)
        .fetch_all(&self.pool)
        .await?;

        let sensors: Vec<Sensor> = rows.into_iter().map(|row| {
            Sensor {
                id: row.get("id"),
                name: row.get("name"),
                data_source: row.get("data_source"),
                location: row.get("location"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        Ok(sensors.into_iter().map(SensorLocation::from).collect())
    }

    pub async fn get_all_sensors(&self) -> Result<Vec<SensorLocation>, SqlxError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                data_source,
                ST_AsText(location) as location,
                created_at,
                updated_at
            FROM sensors 
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let sensors: Vec<Sensor> = rows.into_iter().map(|row| {
            Sensor {
                id: row.get("id"),
                name: row.get("name"),
                data_source: row.get("data_source"),
                location: row.get("location"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        Ok(sensors.into_iter().map(SensorLocation::from).collect())
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

    pub async fn get_sensors_near_point(
        &self,
        latitude: f64,
        longitude: f64,
        radius_km: f64,
    ) -> Result<Vec<SensorLocation>, SqlxError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                data_source,
                ST_AsText(location) as location,
                created_at,
                updated_at
            FROM sensors 
            WHERE ST_DWithin(
                location::geography, 
                ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography,
                $3 * 1000
            )
            ORDER BY ST_Distance(
                location::geography, 
                ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography
            )
            "#
        )
        .bind(longitude)
        .bind(latitude)
        .bind(radius_km * 1000.0) // Convert to meters
        .fetch_all(&self.pool)
        .await?;

        let sensors: Vec<Sensor> = rows.into_iter().map(|row| {
            Sensor {
                id: row.get("id"),
                name: row.get("name"),
                data_source: row.get("data_source"),
                location: row.get("location"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }
        }).collect();

        Ok(sensors.into_iter().map(SensorLocation::from).collect())
    }
}
