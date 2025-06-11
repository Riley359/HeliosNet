use ort::{session::Session, value::Value};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f32,  // Fahrenheit
    pub humidity: f32,     // Percentage 0-100
    pub wind_speed: f32,   // mph
    pub precipitation: f32, // inches in last 7 days
    pub drought_index: f32, // 0-100 scale
}

pub struct RiskModel {
    session: RwLock<Session>,
}

impl RiskModel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load the model file
        let model_path = "model.onnx";        let session = Session::builder()?
            .commit_from_file(model_path)?;
        
        Ok(RiskModel { session: RwLock::new(session) })
    }
      pub fn predict(&self, weather_data: &WeatherData) -> Result<f32, Box<dyn std::error::Error>> {
        // Prepare input data as a flat vector
        let input_data = vec![
            weather_data.temperature,
            weather_data.humidity,
            weather_data.wind_speed,
            weather_data.precipitation,
            weather_data.drought_index,
        ];
        
        // Convert to ONNX Value with correct shape (1, 5)
        let input_tensor = Value::from_array(([1_usize, 5_usize], input_data))?;        // Run inference - use the ort::inputs! macro
        let inputs = ort::inputs!["float_input" => input_tensor];
        let mut session = self.session.write().unwrap();
        let outputs = session.run(inputs)?;// Extract the probability of fire risk (class 1)
        let output_tensor = outputs["output"].try_extract_tensor::<f32>()?;
        let (_shape, data) = output_tensor;
        
        // The model outputs probabilities for both classes [no_risk, risk]
        // We want the probability of risk (index 1)
        let risk_probability = data[1];
        
        Ok(risk_probability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_risk_model_prediction() {
        let model = RiskModel::new().expect("Failed to load model");
        
        // Test with high risk conditions
        let high_risk_weather = WeatherData {
            temperature: 100.0,
            humidity: 10.0,
            wind_speed: 30.0,
            precipitation: 0.0,
            drought_index: 90.0,
        };
        
        let risk = model.predict(&high_risk_weather).expect("Prediction failed");
        assert!(risk > 0.8, "High risk conditions should return high probability");
        
        // Test with low risk conditions
        let low_risk_weather = WeatherData {
            temperature: 50.0,
            humidity: 80.0,
            wind_speed: 2.0,
            precipitation: 3.0,
            drought_index: 10.0,
        };
        
        let risk = model.predict(&low_risk_weather).expect("Prediction failed");
        assert!(risk < 0.2, "Low risk conditions should return low probability");
    }
}
