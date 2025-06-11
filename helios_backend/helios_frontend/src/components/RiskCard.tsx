import React from 'react';
import { RiskPrediction } from '../services/api';

interface RiskCardProps {
  riskData: RiskPrediction | null;
  loading: boolean;
  error: string | null;
}

const RiskCard: React.FC<RiskCardProps> = ({ riskData, loading, error }) => {
  const getRiskColor = (level: string) => {
    switch (level.toUpperCase()) {
      case 'EXTREME': return '#8B0000';
      case 'HIGH': return '#FF0000';
      case 'MODERATE': return '#FFA500';
      case 'LOW': return '#FFFF00';
      case 'MINIMAL': return '#00FF00';
      default: return '#808080';
    }
  };

  const getRiskIcon = (level: string) => {
    switch (level.toUpperCase()) {
      case 'EXTREME': return '🔥🔥🔥';
      case 'HIGH': return '🔥🔥';
      case 'MODERATE': return '🔥';
      case 'LOW': return '⚠️';
      case 'MINIMAL': return '✅';
      default: return '❓';
    }
  };

  if (loading) {
    return (
      <div className="bg-white shadow-lg rounded-lg p-6 mb-4">
        <h2 className="text-xl font-bold mb-4 text-gray-800">Fire Risk Assessment</h2>
        <div className="flex items-center justify-center h-32">
          <div className="animate-pulse text-gray-500">Loading risk assessment...</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-white shadow-lg rounded-lg p-6 mb-4">
        <h2 className="text-xl font-bold mb-4 text-gray-800">Fire Risk Assessment</h2>
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>Error:</strong> {error}
        </div>
      </div>
    );
  }

  if (!riskData) {
    return (
      <div className="bg-white shadow-lg rounded-lg p-6 mb-4">
        <h2 className="text-xl font-bold mb-4 text-gray-800">Fire Risk Assessment</h2>
        <div className="text-gray-500 text-center py-8">
          Click on the map to get fire risk prediction for any location
        </div>
      </div>
    );
  }

  const riskLevel = riskData.risk.level;
  const riskColor = getRiskColor(riskLevel);
  const riskIcon = getRiskIcon(riskLevel);

  return (
    <div className="bg-white shadow-lg rounded-lg p-6 mb-4">
      <h2 className="text-xl font-bold mb-4 text-gray-800">Fire Risk Assessment</h2>
      
      {/* Location */}
      <div className="mb-4">
        <p className="text-sm text-gray-600">
          📍 {riskData.location.latitude.toFixed(4)}, {riskData.location.longitude.toFixed(4)}
        </p>
      </div>

      {/* Risk Level */}
      <div className="mb-6">
        <div 
          className="rounded-lg p-4 text-white font-bold text-center"
          style={{ backgroundColor: riskColor }}
        >
          <div className="text-2xl mb-2">{riskIcon}</div>
          <div className="text-xl">{riskLevel} RISK</div>
          <div className="text-sm opacity-90 mt-1">
            {(riskData.risk.probability * 100).toFixed(1)}% probability
          </div>
        </div>
        <p className="text-sm text-gray-600 mt-2 italic">
          {riskData.risk.description}
        </p>
      </div>

      {/* Weather Conditions */}
      <div className="mb-4">
        <h3 className="font-semibold text-gray-800 mb-2">Current Conditions</h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <div>
            <span className="text-gray-600">Temperature:</span>
            <span className="ml-2 font-medium">{riskData.weather_conditions.temperature.toFixed(1)}°F</span>
          </div>
          <div>
            <span className="text-gray-600">Humidity:</span>
            <span className="ml-2 font-medium">{riskData.weather_conditions.humidity}%</span>
          </div>
          <div>
            <span className="text-gray-600">Wind Speed:</span>
            <span className="ml-2 font-medium">{riskData.weather_conditions.wind_speed.toFixed(1)} mph</span>
          </div>
          <div>
            <span className="text-gray-600">Wind Direction:</span>
            <span className="ml-2 font-medium">{riskData.weather_conditions.wind_direction.toFixed(0)}°</span>
          </div>
        </div>
      </div>

      {/* Model Inputs */}
      <div className="mb-4">
        <h3 className="font-semibold text-gray-800 mb-2">Risk Factors</h3>
        <div className="text-sm space-y-1">
          <div className="flex justify-between">
            <span className="text-gray-600">Drought Index:</span>
            <span className="font-medium">{riskData.model_inputs.drought_index.toFixed(1)}/100</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-600">Recent Precipitation:</span>
            <span className="font-medium">{riskData.model_inputs.precipitation.toFixed(2)}" (7 days)</span>
          </div>
        </div>
      </div>

      {/* Timestamp */}
      <div className="text-xs text-gray-500 mt-4 pt-2 border-t">
        Updated: {new Date(riskData.timestamp).toLocaleString()}
      </div>
    </div>
  );
};

export default RiskCard;
