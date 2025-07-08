use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetQueryRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExoplanetResponse {
    pub data: Vec<HashMap<String, serde_json::Value>>,
}

// Common exoplanet fields for typed access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exoplanet {
    pub pl_name: Option<String>,         // Planet name
    pub hostname: Option<String>,        // Host star name
    pub pl_masse: Option<f64>,          // Planet mass (Earth masses)
    pub pl_rade: Option<f64>,           // Planet radius (Earth radii)
    pub pl_orbper: Option<f64>,         // Orbital period (days)
    pub pl_eqt: Option<f64>,            // Equilibrium temperature (K)
    pub st_teff: Option<f64>,           // Stellar effective temperature (K)
    pub sy_dist: Option<f64>,           // Distance from Earth (pc)
    pub disc_year: Option<i32>,         // Discovery year
    pub discoverymethod: Option<String>, // Discovery method
    pub pl_orbsmax: Option<f64>,       // Orbit semi-major axis (AU)
    pub pl_orbeccen: Option<f64>,      // Eccentricity
    pub st_mass: Option<f64>,           // Stellar mass (solar masses)
    pub st_rad: Option<f64>,            // Stellar radius (solar radii)
}