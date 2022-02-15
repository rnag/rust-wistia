use serde::{Deserialize, Serialize};

/// An object representing the [thumbnail] for a media.
///
/// [thumbnail]: https://wistia.com/support/developers/data-api#medias-response
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u64,
    pub height: u64,
}
