use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    JoinAccept(JoinAccept),
    #[serde(rename = "uplink_message")]
    Uplink(Uplink),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub end_device_ids: EndDeviceIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlation_ids: Vec<String>,
    pub received_at: DateTime<Utc>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndDeviceIds {
    pub device_id: String,
    pub application_ids: HashMap<String, String>,
    pub dev_eui: String,
    pub join_eui: String,
    pub dev_addr: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinAccept {
    pub session_key_id: String,
    pub received_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Uplink {
    pub session_key_id: String,

    #[serde(rename = "f_cnt")]
    pub frame_counter: u32,

    #[serde(default)]
    #[serde(rename = "f_port")]
    pub frame_port: u16,

    #[serde(default)]
    #[serde(rename = "frm_payload")]
    #[serde(with = "crate::Base64Standard")]
    pub frame_payload: Vec<u8>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_payload: Option<Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rx_metadata: Vec<Metadata>,

    pub settings: Settings,

    #[serde(with = "airtime")]
    pub consumed_airtime: Duration,

    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub locations: HashMap<String, Location>,

    pub received_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    // pub data_rate
    pub data_rate_index: u16,
    pub coding_rate: String,
    pub frequency: String,
    pub timestamp: u64,
    pub time: DateTime<Utc>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub source: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub gateway_ids: HashMap<String, String>,
    pub time: DateTime<Utc>,
    pub timestamp: u64,
    pub rssi: f64,
    pub channel_rssi: f64,
    pub snr: f64,
    pub uplink_token: String,
    pub channel_index: u32,
}

mod airtime {
    use super::*;
    use serde::de::Unexpected;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = if let Some(us) = duration.num_microseconds() {
            format!("{}s", us as f64 / 1_000_000.0)
        } else {
            format!("{}s", duration.num_milliseconds() as f64 / 1_000.0)
        };

        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if let Some(value) = value.strip_suffix("s") {
            let s = value.parse::<f64>().map_err(|_| {
                serde::de::Error::invalid_value(Unexpected::Str(&value), &"a floating point value")
            })?;
            return Ok(Duration::microseconds((s * 1_000_000.0) as i64));
        }

        // unknown data format

        Err(serde::de::Error::invalid_value(
            Unexpected::Str(&value),
            &"a floating point value ending with 's'",
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn join_accept_1() {
        let json = include_bytes!("../../test/v3/join_accept.json");
        let uplink: Message = serde_json::from_slice(json).unwrap();

        println!("{:#?}", uplink);

        assert!(matches!(uplink.payload, Payload::JoinAccept(_)));
    }

    #[test]
    pub fn uplink_1() {
        let json = include_bytes!("../../test/v3/uplink.json");
        let uplink: Message = serde_json::from_slice(json).unwrap();

        println!("{:#?}", uplink);

        assert!(matches!(uplink.payload, Payload::Uplink(_)));
    }
}
