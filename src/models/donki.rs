use serde::{Deserialize, Serialize};

// CME - Coronal Mass Ejection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmeEvent {
    #[serde(rename = "activityID")]
    pub activity_id: String,
    pub catalog: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "sourceLocation")]
    pub source_location: Option<String>,
    #[serde(rename = "activeRegionNum")]
    pub active_region_num: Option<u32>,
    pub link: String,
    pub note: Option<String>,
    pub instruments: Vec<Instrument>,
    #[serde(rename = "cmeAnalyses")]
    pub cme_analyses: Option<Vec<CmeAnalysis>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub id: u32,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmeAnalysis {
    pub time21_5: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub half_angle: Option<f64>,
    pub speed: Option<f64>,
    #[serde(rename = "type")]
    pub analysis_type: String,
    pub is_most_accurate: bool,
    pub note: Option<String>,
    pub enlil_list: Option<Vec<EnlilModel>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnlilModel {
    #[serde(rename = "modelCompletionTime")]
    pub model_completion_time: Option<String>,
    pub au: Option<f64>,
    #[serde(rename = "estimatedShockArrivalTime")]
    pub estimated_shock_arrival_time: Option<String>,
    #[serde(rename = "estimatedDuration")]
    pub estimated_duration: Option<f64>,
    pub rmin_re: Option<f64>,
    pub kp_18: Option<f64>,
    pub kp_90: Option<f64>,
    pub kp_135: Option<f64>,
    pub kp_180: Option<f64>,
    pub is_earth_gb: bool,
    pub link: String,
    pub impact_list: Option<Vec<Impact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impact {
    pub is_glancing_blow: bool,
    pub location: String,
    pub arrival_time: String,
}

// GST - Geomagnetic Storm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GstEvent {
    #[serde(rename = "gstID")]
    pub gst_id: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    pub all_kp_index: Vec<KpIndex>,
    pub linked_events: Option<Vec<LinkedEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KpIndex {
    #[serde(rename = "observedTime")]
    pub observed_time: String,
    #[serde(rename = "kpIndex")]
    pub kp_index: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedEvent {
    #[serde(rename = "activityID")]
    pub activity_id: String,
}

// FLR - Solar Flare
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlrEvent {
    #[serde(rename = "flrID")]
    pub flr_id: String,
    pub instruments: Vec<Instrument>,
    #[serde(rename = "beginTime")]
    pub begin_time: String,
    #[serde(rename = "peakTime")]
    pub peak_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "classType")]
    pub class_type: String,
    #[serde(rename = "sourceLocation")]
    pub source_location: Option<String>,
    #[serde(rename = "activeRegionNum")]
    pub active_region_num: Option<u32>,
    pub linked_events: Option<Vec<LinkedEvent>>,
}

// SEP - Solar Energetic Particle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SepEvent {
    #[serde(rename = "sepID")]
    pub sep_id: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub instruments: Vec<Instrument>,
    pub linked_events: Option<Vec<LinkedEvent>>,
}

// IPS - Interplanetary Shock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpsEvent {
    pub catalog: String,
    #[serde(rename = "activityID")]
    pub activity_id: String,
    pub location: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub link: Option<String>,
    pub instruments: Vec<Instrument>,
}

// MPC - Magnetopause Crossing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcEvent {
    #[serde(rename = "mpcID")]
    pub mpc_id: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
}

// RBE - Radiation Belt Enhancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbeEvent {
    #[serde(rename = "rbeID")]
    pub rbe_id: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub instruments: Vec<Instrument>,
}

// HSS - High Speed Stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HssEvent {
    #[serde(rename = "hssID")]
    pub hss_id: String,
    #[serde(rename = "eventTime")]
    pub event_time: String,
    pub instruments: Vec<Instrument>,
    pub link: Option<String>,
}

// WSA+Enlil Simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsaEnlilSimulation {
    #[serde(rename = "simulationID")]
    pub simulation_id: String,
    #[serde(rename = "modelCompletionTime")]
    pub model_completion_time: String,
    pub au: Option<f64>,
    #[serde(rename = "estimatedShockArrivalTime")]
    pub estimated_shock_arrival_time: Option<String>,
    #[serde(rename = "estimatedDuration")]
    pub estimated_duration: Option<f64>,
    pub earth_gb: Option<bool>,
    pub link: String,
    pub impact_list: Option<Vec<Impact>>,
    #[serde(rename = "cmeInputs")]
    pub cme_inputs: Option<Vec<CmeInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmeInput {
    #[serde(rename = "cmeStartTime")]
    pub cme_start_time: String,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub half_angle: f64,
}

// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "messageURL")]
    pub message_url: String,
    #[serde(rename = "messageIssueTime")]
    pub message_issue_time: String,
    #[serde(rename = "messageBody")]
    pub message_body: String,
}