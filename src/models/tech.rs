use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechTransferResponse {
    pub results: Vec<TechItem>,
    pub count: u32,
    pub total: u32,
    pub perpage: u32,
    pub page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "reference_number")]
    pub reference_number: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "client_record_id")]
    pub client_record_id: Option<String>,
    #[serde(rename = "center")]
    pub center: Option<String>,
    #[serde(rename = "date_added")]
    pub date_added: Option<String>,
    #[serde(rename = "date_updated")]
    pub date_updated: Option<String>,
    #[serde(rename = "concepts")]
    pub concepts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "trl")]
    pub trl: Option<String>,
    #[serde(rename = "patent_number")]
    pub patent_number: Option<String>,
    #[serde(rename = "patent_status")]
    pub patent_status: Option<String>,
    #[serde(rename = "patent_expiration_date")]
    pub patent_expiration_date: Option<String>,
    #[serde(flatten)]
    pub item: TechItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Software {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "release_type")]
    pub release_type: Option<String>,
    #[serde(rename = "license")]
    pub license: Option<String>,
    #[serde(rename = "open_source")]
    pub open_source: Option<bool>,
    #[serde(rename = "tariff_code")]
    pub tariff_code: Option<String>,
    #[serde(flatten)]
    pub item: TechItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spinoff {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "company")]
    pub company: Option<String>,
    #[serde(rename = "location")]
    pub location: Option<String>,
    #[serde(rename = "year")]
    pub year: Option<String>,
    #[serde(flatten)]
    pub item: TechItem,
}