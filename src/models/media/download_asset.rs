use crate::models::Media;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct DownloadAssetRequest<'a> {
    pub media_id: Option<&'a str>,
    pub media: Option<Media>,
    pub asset_type: Option<&'a str>,
    pub file_path: Option<PathBuf>,
}

impl<'a> From<&'a str> for DownloadAssetRequest<'a> {
    fn from(media_id: &'a str) -> Self {
        Self {
            media_id: Some(media_id),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a String> for DownloadAssetRequest<'a> {
    fn from(media_id: &'a String) -> Self {
        Self {
            media_id: Some(media_id),
            ..Default::default()
        }
    }
}

impl<'a> From<Media> for DownloadAssetRequest<'a> {
    fn from(media: Media) -> Self {
        Self {
            media: Some(media),
            ..Default::default()
        }
    }
}

impl<'a> DownloadAssetRequest<'a> {
    pub fn asset_type(mut self, asset_type: &'a str) -> Self {
        self.asset_type = Some(asset_type);
        self
    }

    pub fn file_path(mut self, file_path: impl AsRef<Path>) -> Self {
        self.file_path = Some(file_path.as_ref().into());
        self
    }
}
