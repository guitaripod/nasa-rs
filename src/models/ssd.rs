use serde::{Deserialize, Serialize};

// CAD - Close Approach Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadResponse {
    pub signature: CadSignature,
    pub count: String,
    pub fields: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadSignature {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadRequest {
    #[serde(rename = "date-min", skip_serializing_if = "Option::is_none")]
    pub date_min: Option<String>,
    #[serde(rename = "date-max", skip_serializing_if = "Option::is_none")]
    pub date_max: Option<String>,
    #[serde(rename = "dist-max", skip_serializing_if = "Option::is_none")]
    pub dist_max: Option<String>,
    #[serde(rename = "h-min", skip_serializing_if = "Option::is_none")]
    pub h_min: Option<f64>,
    #[serde(rename = "h-max", skip_serializing_if = "Option::is_none")]
    pub h_max: Option<f64>,
    #[serde(rename = "v-inf-min", skip_serializing_if = "Option::is_none")]
    pub v_inf_min: Option<f64>,
    #[serde(rename = "v-inf-max", skip_serializing_if = "Option::is_none")]
    pub v_inf_max: Option<f64>,
    #[serde(rename = "v-rel-min", skip_serializing_if = "Option::is_none")]
    pub v_rel_min: Option<f64>,
    #[serde(rename = "v-rel-max", skip_serializing_if = "Option::is_none")]
    pub v_rel_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pha: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nea: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comet: Option<bool>,
    #[serde(rename = "nea-comet", skip_serializing_if = "Option::is_none")]
    pub nea_comet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullname: Option<bool>,
}

// SBDB - Small Body Database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbdbResponse {
    pub signature: SbdbSignature,
    pub object: SbdbObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_data: Option<Vec<CloseApproachData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phys_par: Option<Vec<PhysicalParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery: Option<DiscoveryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbdbSignature {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbdbObject {
    pub des: String,
    pub name: Option<String>,
    pub fullname: String,
    pub spkid: String,
    pub kind: String,
    pub orbit_id: String,
    pub orbit: OrbitData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitData {
    pub source: String,
    pub cov_epoch: Option<String>,
    pub moid_jup: Option<String>,
    pub t_jup: Option<String>,
    pub condition_code: Option<String>,
    pub not_valid_before: Option<String>,
    pub rms: Option<String>,
    pub model_parms: Option<Vec<String>>,
    pub orbit: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseApproachData {
    pub cd: String,
    pub dist: String,
    pub dist_min: String,
    pub dist_max: String,
    pub v_rel: String,
    pub v_inf: String,
    pub t_sigma_f: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalParameter {
    pub name: String,
    pub value: String,
    pub sigma: Option<String>,
    pub units: Option<String>,
    pub r#ref: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryData {
    pub date: String,
    pub site: String,
    pub who: String,
}

// Sentry - Impact Risk Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentryResponse {
    pub signature: SentrySignature,
    pub count: String,
    pub data: Vec<SentryObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentrySignature {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentryObject {
    pub des: String,
    pub name: Option<String>,
    pub fullname: String,
    pub year_range_min: String,
    pub year_range_max: String,
    pub potential_impacts: String,
    pub impact_probability: String,
    pub vinfinity: String,
    pub absolute_magnitude: String,
    pub estimated_diameter: String,
    pub palermo_scale_ave: String,
    pub palermo_scale_max: String,
    pub torino_scale: String,
    pub last_obs: String,
    pub last_obs_jd: String,
    pub url_nasa_details: String,
    pub url_orbital_elements: String,
    pub is_active_sentry_object: bool,
}

// Fireball
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireballResponse {
    pub signature: FireballSignature,
    pub count: String,
    pub fields: Vec<String>,
    pub data: Vec<Vec<Option<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireballSignature {
    pub source: String,
    pub version: String,
}

// NHATS - Near-Earth Object Human Space Flight Accessible Targets Study
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NhatsResponse {
    pub signature: NhatsSignature,
    pub count: String,
    pub data: Vec<NhatsObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NhatsSignature {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NhatsObject {
    pub des: String,
    pub fullname: String,
    pub min_dv: String,
    pub min_dur: String,
    pub n_via: String,
    pub viable: Vec<ViableTrajectory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViableTrajectory {
    pub dv_total: String,
    pub dur_total: String,
    pub dur_out: String,
    pub dur_ret: String,
    pub dur_at: String,
    pub dep_date: String,
    pub arr_date: String,
    pub ret_date: String,
}

// Scout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutResponse {
    pub signature: ScoutSignature,
    pub object: ScoutObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutSignature {
    pub source: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutObject {
    pub tdes: String,
    pub nobs: u32,
    pub arc: String,
    pub priority: String,
    pub score: String,
    pub rating: String,
    pub neo_score: String,
    pub unc: String,
    pub moid_au: String,
    pub observations: Vec<Observation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub site: String,
    pub time: String,
    pub ra: String,
    pub dec: String,
    pub mag: Option<String>,
}