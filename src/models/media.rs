use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSearchResponse {
    pub collection: MediaCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCollection {
    pub version: String,
    pub href: String,
    pub items: Vec<MediaItem>,
    pub metadata: MediaMetadata,
    pub links: Vec<MediaLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub href: String,
    pub data: Vec<MediaData>,
    pub links: Vec<MediaLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaData {
    pub center: Option<String>,
    pub title: String,
    pub keywords: Option<Vec<String>>,
    pub nasa_id: String,
    pub date_created: String,
    pub media_type: String,
    pub description: String,
    pub photographer: Option<String>,
    pub location: Option<String>,
    pub secondary_creator: Option<String>,
    pub album: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaLink {
    pub rel: String,
    pub prompt: Option<String>,
    pub href: String,
    pub render: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub total_hits: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAssetResponse {
    pub collection: AssetCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCollection {
    pub version: String,
    pub href: String,
    pub items: Vec<AssetItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetItem {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_508: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nasa_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photographer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_end: Option<String>,
}