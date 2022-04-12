use crate::models::Media;
use std::path::{Path, PathBuf};

/// Represents a request to download an [Asset URL].
///
/// [Asset URL]: https://wistia.com/support/developers/asset-urls
///
#[derive(Default)]
pub struct DownloadAssetRequest<'a> {
    /// The Wistia media to download.
    pub media_id: Option<&'a str>,

    /// The Wistia media to download.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rust_wistia::models::DownloadAssetRequest;
    /// use rust_wistia::WistiaClient;
    ///
    /// let media = WistiaClient::from_env()?.get_media("my-id").await?;
    /// let info = DownloadAssetRequest::from(media);
    /// ```    
    pub media: Option<Media>,

    /// The media *asset type* to download.
    pub asset_type: Option<&'a str>,

    /// Sets a local *file path* to save the downloaded media content to.
    pub file_path: Option<PathBuf>,
}

impl<'a> From<&'a str> for DownloadAssetRequest<'a> {
    /// Create a new `DownloadAssetRequest` from a Wistia `media_id`
    fn from(media_id: &'a str) -> Self {
        Self {
            media_id: Some(media_id),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a String> for DownloadAssetRequest<'a> {
    /// Create a new `DownloadAssetRequest` from a Wistia `media_id`
    fn from(media_id: &'a String) -> Self {
        Self {
            media_id: Some(media_id),
            ..Default::default()
        }
    }
}

impl<'a> From<Media> for DownloadAssetRequest<'a> {
    /// Create a new `DownloadAssetRequest` from a `Media` object
    fn from(media: Media) -> Self {
        Self {
            media: Some(media),
            ..Default::default()
        }
    }
}

impl<'a> DownloadAssetRequest<'a> {
    /// Sets the media *asset type* to download from Wistia.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::models::DownloadAssetRequest;
    ///
    /// let info = DownloadAssetRequest::from("my-video-id").asset_type("OriginalFile");
    /// ```    
    pub fn asset_type(mut self, asset_type: &'a str) -> Self {
        self.asset_type = Some(asset_type);
        self
    }

    /// Sets a local *file path* to save the downloaded media content to.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use rust_wistia::models::DownloadAssetRequest;
    /// use rust_wistia::WistiaClient;
    ///
    /// let info = DownloadAssetRequest::from("my-video-id").file_path("./my-file.mp4");
    /// let _ = WistiaClient::from_env()?.download_asset(info).await?;
    /// ```    
    pub fn file_path(mut self, file_path: impl AsRef<Path>) -> Self {
        self.file_path = Some(file_path.as_ref().into());
        self
    }
}
