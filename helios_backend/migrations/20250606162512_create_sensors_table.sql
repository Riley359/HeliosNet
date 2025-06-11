-- Create PostGIS extension if it doesn't exist
CREATE EXTENSION IF NOT EXISTS postgis;

-- Create the sensors table with geospatial capabilities
CREATE TABLE IF NOT EXISTS sensors (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    data_source VARCHAR(100) NOT NULL DEFAULT 'AirNow',
    location GEOMETRY(Point, 4326) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create spatial index for efficient location queries
CREATE INDEX IF NOT EXISTS idx_sensors_location ON sensors USING GIST (location);

-- Create index on data_source for filtering
CREATE INDEX IF NOT EXISTS idx_sensors_data_source ON sensors (data_source);
