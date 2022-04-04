use super::{MediaStatus, MediaType, Thumbnail};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    #[serde(rename = "hashed_id")]
    pub hashed_id: String,
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: MediaType,
    pub created: String,
    pub updated: String,
    /// Note: only videos have this attribute set; thumbnails and other
    /// medias don't.
    pub duration: Option<f64>,
    pub status: MediaStatus,
    pub description: String,
    pub progress: f64,
    pub thumbnail: Thumbnail,
    pub section: Option<String>,
}
