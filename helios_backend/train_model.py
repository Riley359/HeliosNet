#!/usr/bin/env python3
"""
Script to execute the fire risk model training notebook and generate the ONNX model.
"""

import pandas as pd
import numpy as np
import matplotlib
matplotlib.use('Agg')  # Use non-interactive backend
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split
from sklearn.metrics import classification_report, confusion_matrix, accuracy_score
from sklearn.preprocessing import StandardScaler
from sklearn.pipeline import Pipeline
import warnings
warnings.filterwarnings('ignore')

# For ONNX export
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType

print("Starting fire risk model training...")

# Create synthetic fire risk dataset based on realistic weather patterns
np.random.seed(42)

# Generate 10000 samples
n_samples = 10000

# Weather features that influence fire risk
data = {
    'temperature': np.random.normal(75, 20, n_samples),  # Fahrenheit
    'humidity': np.random.beta(2, 3, n_samples) * 100,   # Percentage 0-100
    'wind_speed': np.random.exponential(8, n_samples),   # mph
    'precipitation': np.random.exponential(0.1, n_samples),  # inches in last 7 days
    'drought_index': np.random.beta(2, 5, n_samples) * 100,  # 0-100 scale
}

df = pd.DataFrame(data)

# Ensure realistic ranges
df['temperature'] = np.clip(df['temperature'], 20, 120)
df['humidity'] = np.clip(df['humidity'], 5, 95)
df['wind_speed'] = np.clip(df['wind_speed'], 0, 50)
df['precipitation'] = np.clip(df['precipitation'], 0, 5)
df['drought_index'] = np.clip(df['drought_index'], 0, 100)

# Create fire risk labels based on realistic conditions
def calculate_fire_risk(row):
    risk_score = 0
    
    # Temperature contribution (higher = more risk)
    if row['temperature'] > 85:
        risk_score += 2
    elif row['temperature'] > 75:
        risk_score += 1
    
    # Humidity contribution (lower = more risk)
    if row['humidity'] < 20:
        risk_score += 3
    elif row['humidity'] < 40:
        risk_score += 2
    elif row['humidity'] < 60:
        risk_score += 1
    
    # Wind speed contribution (higher = more risk)
    if row['wind_speed'] > 20:
        risk_score += 2
    elif row['wind_speed'] > 10:
        risk_score += 1
    
    # Precipitation contribution (lower = more risk)
    if row['precipitation'] < 0.1:
        risk_score += 2
    elif row['precipitation'] < 0.5:
        risk_score += 1
    
    # Drought index contribution (higher = more risk)
    if row['drought_index'] > 70:
        risk_score += 2
    elif row['drought_index'] > 50:
        risk_score += 1
    
    # Add some randomness to make it more realistic
    risk_score += np.random.normal(0, 0.5)
    
    # Convert to binary classification (high risk = 1, low risk = 0)
    return 1 if risk_score > 5 else 0

df['fire_risk'] = df.apply(calculate_fire_risk, axis=1)

print(f"Dataset created with {len(df)} samples")
print(f"Fire risk distribution:")
print(df['fire_risk'].value_counts())
print(f"High risk percentage: {df['fire_risk'].mean()*100:.1f}%")

# Prepare features and target
X = df[['temperature', 'humidity', 'wind_speed', 'precipitation', 'drought_index']]
y = df['fire_risk']

# Split the data
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42, stratify=y)

print(f"Training set size: {len(X_train)}")
print(f"Test set size: {len(X_test)}")

# Scale the features for better model performance
scaler = StandardScaler()
X_train_scaled = scaler.fit_transform(X_train)
X_test_scaled = scaler.transform(X_test)

# Train Random Forest model
rf_model = RandomForestClassifier(
    n_estimators=100,
    max_depth=10,
    min_samples_split=5,
    min_samples_leaf=2,
    random_state=42
)

rf_model.fit(X_train_scaled, y_train)

# Make predictions
y_pred = rf_model.predict(X_test_scaled)
y_pred_proba = rf_model.predict_proba(X_test_scaled)[:, 1]

# Evaluate the model
accuracy = accuracy_score(y_test, y_pred)
print(f"\nModel Accuracy: {accuracy:.3f}")

print("\nClassification Report:")
print(classification_report(y_test, y_pred))

# Feature importance
feature_importance = pd.DataFrame({
    'feature': X.columns,
    'importance': rf_model.feature_importances_
}).sort_values('importance', ascending=False)

print("\nFeature Importance:")
print(feature_importance)

# Create a pipeline with scaler and model
pipeline = Pipeline([
    ('scaler', scaler),
    ('classifier', rf_model)
])

# Fit the pipeline (this just sets up the pipeline structure)
pipeline.fit(X_train, y_train)

# Define the input type for ONNX conversion
initial_type = [('float_input', FloatTensorType([None, 5]))]

# Convert to ONNX
print("\nConverting model to ONNX format...")
onnx_model = convert_sklearn(pipeline, initial_types=initial_type)

# Save the ONNX model
with open('model.onnx', 'wb') as f:
    f.write(onnx_model.SerializeToString())

print("Model successfully exported to model.onnx")

# Test the exported model with sample predictions
test_scenarios = {
    'Extreme High Risk': [100, 10, 30, 0, 90],
    'High Risk': [90, 20, 20, 0.1, 70],
    'Moderate Risk': [75, 45, 10, 0.5, 50],
    'Low Risk': [65, 65, 5, 1.5, 30],
    'Very Low Risk': [50, 80, 2, 3.0, 10]
}

print("\nModel Validation - Test Scenarios:")
print("===================================")
print(f"{'Scenario':<20} {'Temp':<6} {'Humid':<6} {'Wind':<6} {'Precip':<7} {'Drought':<7} {'Risk':<6}")
print("-" * 70)

for scenario_name, conditions in test_scenarios.items():
    risk_prob = pipeline.predict_proba([conditions])[0, 1]
    print(f"{scenario_name:<20} {conditions[0]:<6.0f} {conditions[1]:<6.0f} {conditions[2]:<6.0f} {conditions[3]:<7.2f} {conditions[4]:<7.0f} {risk_prob:<6.3f}")

print("\nModel ready for integration into Rust backend!")
