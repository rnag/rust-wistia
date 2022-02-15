use serde::{Deserialize, Serialize};

/// Media files return a response attribute called status.
///
/// After upload is complete, media files must be processed. Status indicates
/// which stage in processing the file is at.
///
/// See also: [Media Status][]
///
/// [Media Status]: https://wistia.com/support/developers/data-api#media-status
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaStatus {
    /// **queued**: the file is waiting in the queue to be processed
    Queued,
    /// **processing**: the file is actively being processed
    Processing,
    /// **ready**: the file has been fully processed and is ready for
    /// embedding and viewing.
    Ready,
    /// **failed**: the file was unable to be processed (usually a
    /// [format or size error](https://wistia.com/support/uploading/export-settings))
    Failed,
}

impl Default for MediaStatus {
    fn default() -> Self {
        Self::Queued
    }
}
