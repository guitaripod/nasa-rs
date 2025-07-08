use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthImageryRequest {
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dim: Option<f64>,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthAssetsRequest {
    pub lat: f64,
    pub lon: f64,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dim: Option<f64>,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarthAsset {
    pub date: String,
    pub id: String,
    pub url: String,
    pub cloud_score: Option<f64>,
}