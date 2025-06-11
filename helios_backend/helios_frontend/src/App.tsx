import { useState, useEffect } from 'react';
import './App.css';
import EnvironmentalMap from './components/EnvironmentalMap';
import AirQualityCard from './components/AirQualityCard';
import WeatherCard from './components/WeatherCard';
import RiskCard from './components/RiskCard';
import { environmentalAPI, type EnvironmentalData, type RiskPrediction } from './services/api';

function App() {
  const [environmentalData, setEnvironmentalData] = useState<EnvironmentalData | null>(null);
  const [riskData, setRiskData] = useState<RiskPrediction | null>(null);
  const [loading, setLoading] = useState(true);
  const [riskLoading, setRiskLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [riskError, setRiskError] = useState<string | null>(null);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  const fetchEnvironmentalData = async (lat?: number, lon?: number) => {
    try {
      setLoading(true);
      setError(null);
      
      const data = lat && lon 
        ? await environmentalAPI.getDataByCoords(lat, lon)
        : await environmentalAPI.getCurrentData();
      
      setEnvironmentalData(data);
      setLastUpdated(new Date());
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch data');
      console.error('Error fetching environmental data:', err);
    } finally {
      setLoading(false);
    }
  };

  const fetchRiskPrediction = async (lat: number, lon: number) => {
    try {
      setRiskLoading(true);
      setRiskError(null);
      
      const data = await environmentalAPI.getRiskPrediction(lat, lon);
      setRiskData(data);
    } catch (err) {
      setRiskError(err instanceof Error ? err.message : 'Failed to fetch risk prediction');
      console.error('Error fetching risk prediction:', err);
    } finally {
      setRiskLoading(false);
    }
  };

  const handleMapClick = (lat: number, lng: number) => {
    fetchEnvironmentalData(lat, lng);
    fetchRiskPrediction(lat, lng);
  };

  const handleRefresh = () => {
    fetchEnvironmentalData();
  };

  useEffect(() => {
    // Initial data fetch
    fetchEnvironmentalData();

    // Set up auto-refresh every 5 minutes
    const interval = setInterval(() => {
      fetchEnvironmentalData();
    }, 5 * 60 * 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="app">
      <header className="app-header">
        <h1>🌍 HeliosNet Environmental Monitor</h1>
        <p className="app-subtitle">Real-time environmental data for Altamont, Oregon</p>
        <div className="header-controls">
          <button 
            className="refresh-btn" 
            onClick={handleRefresh}
            disabled={loading}
          >
            {loading ? '⟳ Loading...' : '🔄 Refresh Data'}
          </button>
          {lastUpdated && (
            <span className="last-updated">
              Last updated: {lastUpdated.toLocaleTimeString()}
            </span>
          )}
        </div>
      </header>

      <main className="app-main">
        {error && (
          <div className="error-banner">
            <span>⚠️ {error}</span>
            <button onClick={handleRefresh}>Try Again</button>
          </div>
        )}

        <div className="dashboard-grid">
          <div className="map-section">
            <EnvironmentalMap 
              data={environmentalData}
              loading={loading}
              onMapClick={handleMapClick}
            />
            {!loading && environmentalData && (
              <p className="map-instructions">
                Click anywhere on the map to get environmental data and fire risk assessment for that location
              </p>
            )}
          </div>

          <div className="data-section">
            <RiskCard 
              riskData={riskData}
              loading={riskLoading}
              error={riskError}
            />
            {environmentalData ? (
              <>
                <AirQualityCard data={environmentalData.air_quality} />
                <WeatherCard data={environmentalData.weather} />
              </>
            ) : !loading && (
              <div className="no-data">
                <h3>No data available</h3>
                <p>Click the refresh button to try loading data again.</p>
              </div>
            )}
          </div>
        </div>
      </main>

      <footer className="app-footer">
        <p>Data provided by AirNow API and OpenWeatherMap API</p>
      </footer>
    </div>
  );
}

export default App;
