import React from 'react';
import type { AirQualityData } from '../services/api';

interface AirQualityCardProps {
  data: AirQualityData;
}

const AirQualityCard: React.FC<AirQualityCardProps> = ({ data }) => {
  const getAQIColor = (aqi: number): string => {
    if (aqi <= 50) return '#00e400'; // Good
    if (aqi <= 100) return '#ffff00'; // Moderate
    if (aqi <= 150) return '#ff7e00'; // Unhealthy for Sensitive Groups
    if (aqi <= 200) return '#ff0000'; // Unhealthy
    if (aqi <= 300) return '#8f3f97'; // Very Unhealthy
    return '#7e0023'; // Hazardous
  };

  const getHealthMessage = (aqi: number): string => {
    if (aqi <= 50) return 'Air quality is satisfactory, and air pollution poses little or no risk.';
    if (aqi <= 100) return 'Air quality is acceptable. However, there may be a risk for some people, particularly those who are unusually sensitive to air pollution.';
    if (aqi <= 150) return 'Members of sensitive groups may experience health effects. The general public is less likely to be affected.';
    if (aqi <= 200) return 'Some members of the general public may experience health effects; members of sensitive groups may experience more serious health effects.';
    if (aqi <= 300) return 'Health alert: The risk of health effects is increased for everyone.';
    return 'Health warning of emergency conditions: everyone is more likely to be affected.';
  };

  return (
    <div className="environmental-card air-quality-card">
      <h3>Air Quality Index</h3>
      <div className="aqi-display">
        <div 
          className="aqi-circle"
          style={{ backgroundColor: getAQIColor(data.aqi) }}
        >
          <span className="aqi-value">{data.aqi}</span>
        </div>
        <div className="aqi-info">
          <h4 className="aqi-category">{data.category}</h4>
          <p className="aqi-location">{data.location}</p>
        </div>
      </div>
      <p className="health-message">{getHealthMessage(data.aqi)}</p>
      <p className="timestamp">Last updated: {new Date(data.timestamp).toLocaleString()}</p>
    </div>
  );
};

export default AirQualityCard;