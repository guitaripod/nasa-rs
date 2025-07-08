use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicImage {
    pub identifier: String,
    pub caption: String,
    pub image: String,
    pub version: String,
    pub centroid_coordinates: Coordinates,
    pub dscovr_j2000_position: Position,
    pub lunar_j2000_position: Position,
    pub sun_j2000_position: Position,
    pub attitude_quaternions: Quaternions,
    pub date: String,
    pub coords: ImageCoords,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quaternions {
    pub q0: f64,
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageCoords {
    pub centroid_coordinates: Coordinates,
    pub dscovr_j2000_position: Position,
    pub lunar_j2000_position: Position,
    pub sun_j2000_position: Position,
    pub attitude_quaternions: Quaternions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpicDate {
    pub date: String,
}