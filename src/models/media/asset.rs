use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub content_type: String,
    pub file_size: u64,
    pub height: u64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub width: u64,
}
