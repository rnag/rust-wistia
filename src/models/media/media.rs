use super::{Asset, MediaStatus, MediaType, ProjectInfo, Thumbnail};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
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
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub progress: f64,
    pub thumbnail: Thumbnail,
    pub project: ProjectInfo,
    #[serde(skip_serializing)]
    pub embed_code: String,
    pub assets: Vec<Asset>,
    pub section: Option<String>,
}
