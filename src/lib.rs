//! Mappings for APIs of *The Things Network*.

pub mod v2;
pub mod v3;

use base64::STANDARD;
use base64_serde::base64_serde_type;

base64_serde_type!(Base64Standard, STANDARD);
