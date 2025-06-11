import axios from 'axios';

const API_BASE_URL = 'http://localhost:8080';

export interface AirQualityData {
  aqi: number;
  category: string;
  location: string;
  timestamp: string;
}

export interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  wind_direction: number;
}

export interface EnvironmentalData {
  air_quality: AirQualityData;
  weather: WeatherData;
  location: {
    latitude: number;
    longitude: number;
  };
}

export interface SensorLocation {
  id: number;
  name: string;
  data_source: string;
  latitude: number;
  longitude: number;
}

export interface SensorsResponse {
  count: number;
  sensors: SensorLocation[];
  timestamp: string;
}

export interface RiskPrediction {
  location: {
    latitude: number;
    longitude: number;
  };
  risk: {
    probability: number;
    level: string;
    description: string;
  };
  weather_conditions: {
    temperature: number;
    humidity: number;
    wind_speed: number;
    wind_direction: number;
  };
  model_inputs: {
    temperature: number;
    humidity: number;
    wind_speed: number;
    precipitation: number;
    drought_index: number;
  };
  timestamp: string;
}

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
});

export const environmentalAPI = {
  // Get current environmental data for Altamont, Oregon
  getCurrentData: async (): Promise<EnvironmentalData> => {
    try {
      const response = await api.get('/environmental-data');
      return response.data;
    } catch (error) {
      console.error('Failed to fetch environmental data:', error);
      throw new Error('Failed to fetch environmental data');
    }
  },

  // Get data for specific coordinates
  getDataByCoords: async (lat: number, lon: number): Promise<EnvironmentalData> => {
    try {
      const response = await api.get(`/environmental-data?lat=${lat}&lon=${lon}`);
      return response.data;
    } catch (error) {
      console.error('Failed to fetch environmental data for coordinates:', error);
      throw new Error('Failed to fetch environmental data');
    }
  },

  // Health check
  healthCheck: async (): Promise<{ status: string }> => {
    try {
      const response = await api.get('/health');
      return response.data;
    } catch (error) {
      console.error('Health check failed:', error);
      throw new Error('Backend service unavailable');
    }
  },

  // Get sensor data for specific bounds
  getSensorsByBounds: async (
    minLat: number, 
    minLon: number, 
    maxLat: number, 
    maxLon: number
  ): Promise<SensorsResponse> => {
    try {
      const response = await api.get(
        `/api/sensors?min_lat=${minLat}&min_lon=${minLon}&max_lat=${maxLat}&max_lon=${maxLon}`
      );
      return response.data;
    } catch (error) {
      console.error('Failed to fetch sensors by bounds:', error);
      throw new Error('Failed to fetch sensors');
    }
  },

  // Get all sensors
  getAllSensors: async (): Promise<SensorsResponse> => {
    try {
      const response = await api.get('/api/sensors');
      return response.data;
    } catch (error) {
      console.error('Failed to fetch all sensors:', error);
      throw new Error('Failed to fetch sensors');
    }
  },

  // Get fire risk prediction for specific coordinates
  getRiskPrediction: async (lat: number, lon: number): Promise<RiskPrediction> => {
    try {
      const response = await api.get(`/api/risk/point?lat=${lat}&lon=${lon}`);
      return response.data;
    } catch (error) {
      console.error('Failed to fetch risk prediction:', error);
      throw new Error('Failed to fetch risk prediction');
    }
  },
};