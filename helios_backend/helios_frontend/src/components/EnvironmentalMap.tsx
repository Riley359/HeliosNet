import React, { useEffect, useState } from 'react';
import { MapContainer, TileLayer, Marker, Popup, useMapEvents } from 'react-leaflet';
import type { LatLngExpression } from 'leaflet';
import 'leaflet/dist/leaflet.css';
import type { EnvironmentalData, SensorLocation } from '../services/api';
import { environmentalAPI } from '../services/api';

// Fix for default markers in react-leaflet
import L from 'leaflet';

let DefaultIcon = L.divIcon({
  html: `<svg width="25" height="41" viewBox="0 0 25 41" xmlns="http://www.w3.org/2000/svg">
    <path d="M12.5 0C5.6 0 0 5.6 0 12.5c0 6.9 12.5 28.5 12.5 28.5s12.5-21.6 12.5-28.5C25 5.6 19.4 0 12.5 0z" fill="#3388ff"/>
    <circle cx="12.5" cy="12.5" r="6" fill="white"/>
  </svg>`,
  className: 'custom-div-icon',
  iconSize: [25, 41],
  iconAnchor: [12, 41],
});

let SensorIcon = L.divIcon({
  html: `<svg width="20" height="20" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
    <circle cx="10" cy="10" r="8" fill="#ff6b35" stroke="white" stroke-width="2"/>
    <circle cx="10" cy="10" r="4" fill="white"/>
  </svg>`,
  className: 'sensor-div-icon',
  iconSize: [20, 20],
  iconAnchor: [10, 10],
});

L.Marker.prototype.options.icon = DefaultIcon;

interface EnvironmentalMapProps {
  data: EnvironmentalData | null;
  loading: boolean;
  onMapClick?: (lat: number, lng: number) => void;
}

// Custom component to handle map clicks
function MapClickHandler({ onMapClick }: { onMapClick?: (lat: number, lng: number) => void }) {
  useMapEvents({
    click(e) {
      if (onMapClick) {
        onMapClick(e.latlng.lat, e.latlng.lng);
      }
    },
  });
  return null;
}

const EnvironmentalMap: React.FC<EnvironmentalMapProps> = ({ data, loading, onMapClick }) => {
  // Altamont, Oregon coordinates
  const altamontCenter: LatLngExpression = [44.1292, -121.7689];
  
  // State for sensor data
  const [sensors, setSensors] = useState<SensorLocation[]>([]);
  const [sensorsLoading, setSensorsLoading] = useState(false);

  // Load all sensors on component mount
  useEffect(() => {
    const loadSensors = async () => {
      setSensorsLoading(true);
      try {
        const response = await environmentalAPI.getAllSensors();
        setSensors(response.sensors);
      } catch (error) {
        console.error('Failed to load sensors:', error);
      } finally {
        setSensorsLoading(false);
      }
    };

    loadSensors();
  }, []);
  
  const getAQIColor = (aqi: number): string => {
    if (aqi <= 50) return '#00e400'; // Good
    if (aqi <= 100) return '#ffff00'; // Moderate
    if (aqi <= 150) return '#ff7e00'; // Unhealthy for Sensitive Groups
    if (aqi <= 200) return '#ff0000'; // Unhealthy
    if (aqi <= 300) return '#8f3f97'; // Very Unhealthy
    return '#7e0023'; // Hazardous
  };

  return (
    <div className="map-container">
      <MapContainer
        center={altamontCenter}
        zoom={10}
        style={{ height: '500px', width: '100%' }}
      >
        <MapClickHandler onMapClick={onMapClick} />
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
          {data && (
          <Marker 
            position={[data.location.latitude, data.location.longitude]}
          >
            <Popup>
              <div className="popup-content">
                <h3>Environmental Data</h3>
                <div className="data-section">
                  <h4>Air Quality</h4>
                  <p>
                    <span 
                      className="aqi-badge" 
                      style={{ backgroundColor: getAQIColor(data.air_quality.aqi) }}
                    >
                      AQI: {data.air_quality.aqi}
                    </span>
                  </p>
                  <p>Category: {data.air_quality.category}</p>
                </div>
                
                <div className="data-section">
                  <h4>Weather</h4>
                  <p>Temperature: {data.weather.temperature.toFixed(1)}°C</p>
                  <p>Humidity: {data.weather.humidity}%</p>
                  <p>Wind: {data.weather.wind_speed.toFixed(1)} m/s</p>
                  <p>Direction: {data.weather.wind_direction.toFixed(0)}°</p>
                </div>
              </div>
            </Popup>
          </Marker>
        )}

        {/* Sensor markers */}
        {sensors.map((sensor) => (
          <Marker
            key={sensor.id}
            position={[sensor.latitude, sensor.longitude]}
            icon={SensorIcon}
          >
            <Popup>
              <div className="popup-content">
                <h3>{sensor.name}</h3>
                <p><strong>Data Source:</strong> {sensor.data_source}</p>
                <p><strong>Location:</strong> {sensor.latitude.toFixed(4)}, {sensor.longitude.toFixed(4)}</p>
                <p><em>Sensor ID: {sensor.id}</em></p>
              </div>
            </Popup>
          </Marker>
        ))}
      </MapContainer>
        {(loading || sensorsLoading) && (
        <div className="map-loading-overlay">
          <div className="loading-spinner">
            {loading && "Loading environmental data..."}
            {sensorsLoading && "Loading sensor locations..."}
          </div>
        </div>
      )}
    </div>
  );
};

export default EnvironmentalMap;