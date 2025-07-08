pub mod apod;
pub mod neo;
pub mod donki;
pub mod earth;
pub mod epic;
pub mod mars;
pub mod tech;
pub mod media;
pub mod exoplanets;
pub mod ssd;

// Common types used across multiple APIs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: String,
    pub cached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub total_pages: u32,
    pub total_items: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
}