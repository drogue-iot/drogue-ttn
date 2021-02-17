//! Mappings for the HTTP integration.

use base64::STANDARD;
use base64_serde::base64_serde_type;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

base64_serde_type!(Base64Standard, STANDARD);

/// Uplink message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Uplink {
    pub app_id: String,
    pub dev_id: String,
    pub hardware_serial: String,
    pub port: u16,
    pub counter: u32,
    #[serde(default)]
    pub is_retry: bool,
    #[serde(default)]
    pub confirmed: bool,

    #[serde(skip_serializing_if = "Value::is_null", default)]
    pub payload_fields: Value,
    #[serde(with = "Base64Standard")]
    pub payload_raw: Vec<u8>,

    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub downlink_url: Option<Url>,
}

/// Metadata section.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub time: DateTime<Utc>,
    pub frequency: f64,
    pub modulation: String,
    pub data_rate: Option<String>,
    pub bit_rate: Option<u32>,
    pub coding_rate: String,

    #[serde(flatten)]
    pub coordinates: Option<Coordinates>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub gateways: Vec<Gateway>,
}

/// Gateway information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gateway {
    pub gtw_id: String,
    #[serde(default)]
    pub channel: u32,
    pub rssi: f64,
    #[serde(default)]
    pub snr: f64,
    #[serde(default)]
    pub rf_chain: i32,
    #[serde(flatten)]
    pub coordinates: Option<Coordinates>,
}

/// Location coordinates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64,
}
