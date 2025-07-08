use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarsPhotosResponse {
    pub photos: Vec<MarsPhoto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarsPhoto {
    pub id: u32,
    pub sol: u32,
    pub camera: Camera,
    pub img_src: String,
    pub earth_date: String,
    pub rover: Rover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub id: u32,
    pub name: String,
    pub rover_id: u32,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rover {
    pub id: u32,
    pub name: String,
    pub landing_date: String,
    pub launch_date: String,
    pub status: String,
    pub max_sol: u32,
    pub max_date: String,
    pub total_photos: u32,
    pub cameras: Vec<CameraInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraInfo {
    pub name: String,
    pub full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarsManifest {
    pub photo_manifest: PhotoManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoManifest {
    pub name: String,
    pub landing_date: String,
    pub launch_date: String,
    pub status: String,
    pub max_sol: u32,
    pub max_date: String,
    pub total_photos: u32,
    pub photos: Vec<SolSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolSummary {
    pub sol: u32,
    pub earth_date: String,
    pub total_photos: u32,
    pub cameras: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarsPhotoRequest {
    pub rover: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sol: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}