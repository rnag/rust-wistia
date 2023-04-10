use super::{Asset, MediaStatus, MediaType, ProjectInfo, Thumbnail};
use crate::constants::*;
use crate::{Result, RustWistiaError};

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
    #[serde(default)]
    pub embed_code: String,
    pub assets: Vec<Asset>,
    pub section: Option<String>,
    #[serde(default)]
    pub archived: bool,
}

impl Media {
    /// Retrieve the *asset URL* for the original source media that was uploaded.
    pub fn source_url(&self) -> Result<String> {
        self.asset_url(None)
    }

    /// Retrieve the *asset URL* (default: **HTTPS**) for a specified `asset_type`, which
    /// defaults to the original source media if not provided.
    pub fn asset_url<'a>(&'a self, asset_type: impl Into<Option<&'a str>>) -> Result<String> {
        let url = self.asset_url_insecure(asset_type)?;

        let (_, id) = url.rsplit_once('/').unwrap();
        let id = match id.rsplit_once(".bin") {
            Some((id, _)) => id,
            None => id,
        };

        Ok(format!(
            "https://embed-ssl.wistia.com/deliveries/{id}/{DEFAULT_FILENAME}"
        ))
    }

    /// Retrieve the *asset URL* (default: **HTTP**) for a specified `asset_type`, which
    /// defaults to the original source media if not provided.
    pub fn asset_url_insecure<'a>(
        &'a self,
        asset_type: impl Into<Option<&'a str>>,
    ) -> Result<&'a str> {
        let r#type = asset_type.into().unwrap_or(ORIGINAL_ASSET);

        for asset in self.assets.iter() {
            if asset.type_field == r#type {
                return Ok(&asset.url);
            }
        }

        Err(RustWistiaError::AssetNotFound {
            r#type: r#type.to_string(),
            video_id: self.hashed_id.clone(),
            valid_types: self.assets.iter().map(|a| a.type_field.clone()).collect(),
        })
    }
}
