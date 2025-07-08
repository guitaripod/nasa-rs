use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoFeedResponse {
    pub links: Links,
    pub element_count: u32,
    pub near_earth_objects: HashMap<String, Vec<Neo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neo {
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub nasa_jpl_url: String,
    pub absolute_magnitude_h: f64,
    pub estimated_diameter: EstimatedDiameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<CloseApproach>,
    pub is_sentry_object: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedDiameter {
    pub kilometers: DiameterRange,
    pub meters: DiameterRange,
    pub miles: DiameterRange,
    pub feet: DiameterRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiameterRange {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseApproach {
    pub close_approach_date: String,
    pub close_approach_date_full: String,
    pub epoch_date_close_approach: i64,
    pub relative_velocity: VelocityData,
    pub miss_distance: DistanceData,
    pub orbiting_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityData {
    pub kilometers_per_second: String,
    pub kilometers_per_hour: String,
    pub miles_per_hour: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceData {
    pub astronomical: String,
    pub lunar: String,
    pub kilometers: String,
    pub miles: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    pub next: Option<String>,
    pub prev: Option<String>,
    #[serde(rename = "self")]
    pub self_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoBrowseResponse {
    pub links: Links,
    pub page: PageInfo,
    pub near_earth_objects: Vec<Neo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub size: u32,
    pub total_elements: u32,
    pub total_pages: u32,
    pub number: u32,
}