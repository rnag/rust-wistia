use crate::utils::is_default;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub content_type: String,
    pub file_size: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    /// Note: `height` will **not** be populated for audio (or alternate audio) files
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub height: u64,
    /// Note: `width` will **not** be populated for audio (or alternate audio) files
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub width: u64,
}
