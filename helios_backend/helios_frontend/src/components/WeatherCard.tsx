import React from 'react';
import type { WeatherData } from '../services/api';

interface WeatherCardProps {
  data: WeatherData;
}

const WeatherCard: React.FC<WeatherCardProps> = ({ data }) => {
  const getWindDirection = (degrees: number): string => {
    const directions = ['N', 'NNE', 'NE', 'ENE', 'E', 'ESE', 'SE', 'SSE', 'S', 'SSW', 'SW', 'WSW', 'W', 'WNW', 'NW', 'NNW'];
    const index = Math.round(degrees / 22.5) % 16;
    return directions[index];
  };

  const getTemperatureColor = (temp: number): string => {
    if (temp <= 0) return '#0066cc'; // Very cold - blue
    if (temp <= 10) return '#0099ff'; // Cold - light blue
    if (temp <= 20) return '#66cc00'; // Cool - green
    if (temp <= 30) return '#ffcc00'; // Warm - yellow
    if (temp <= 35) return '#ff6600'; // Hot - orange
    return '#ff0000'; // Very hot - red
  };

  const getHumidityLevel = (humidity: number): string => {
    if (humidity < 30) return 'Low';
    if (humidity < 60) return 'Comfortable';
    if (humidity < 80) return 'High';
    return 'Very High';
  };

  const getWindSpeedLevel = (speed: number): string => {
    if (speed < 2) return 'Calm';
    if (speed < 6) return 'Light Breeze';
    if (speed < 12) return 'Moderate Breeze';
    if (speed < 20) return 'Strong Breeze';
    return 'High Wind';
  };

  return (
    <div className="environmental-card weather-card">
      <h3>Weather Conditions</h3>
      
      <div className="weather-grid">
        <div className="weather-item temperature-item">
          <div className="weather-icon">🌡️</div>
          <div className="weather-info">
            <h4>Temperature</h4>
            <span 
              className="weather-value temperature-value"
              style={{ color: getTemperatureColor(data.temperature) }}
            >
              {data.temperature.toFixed(1)}°C
            </span>
            <span className="weather-unit">({(data.temperature * 9/5 + 32).toFixed(1)}°F)</span>
          </div>
        </div>

        <div className="weather-item humidity-item">
          <div className="weather-icon">💧</div>
          <div className="weather-info">
            <h4>Humidity</h4>
            <span className="weather-value">{data.humidity}%</span>
            <span className="weather-level">{getHumidityLevel(data.humidity)}</span>
          </div>
        </div>

        <div className="weather-item wind-item">
          <div className="weather-icon">💨</div>
          <div className="weather-info">
            <h4>Wind</h4>
            <span className="weather-value">{data.wind_speed.toFixed(1)} m/s</span>
            <span className="weather-level">{getWindSpeedLevel(data.wind_speed)}</span>
          </div>
        </div>

        <div className="weather-item direction-item">
          <div className="weather-icon">🧭</div>
          <div className="weather-info">
            <h4>Direction</h4>
            <span className="weather-value">{getWindDirection(data.wind_direction)}</span>
            <span className="weather-unit">({data.wind_direction.toFixed(0)}°)</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default WeatherCard;