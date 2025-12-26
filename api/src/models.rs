use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureRequest {
    pub day: u32,
    pub month: u32,
    pub samples: u32,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureResponse {
    pub day: u32,
    pub month: u32,
    pub samples_requested: u32,
    pub samples_found: u32,
    pub average_temperature: f64,
    pub years_included: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecipitationRequest {
    pub month: u32,
    pub samples: u32,
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrecipitationResponse {
    pub month: u32,
    pub samples_requested: u32,
    pub samples_found: u32,
    pub total_precipitation: f64,
    pub years_included: Vec<i32>,
}
