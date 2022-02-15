use serde::{Deserialize, Serialize};

/// A value which represents the type of media.
///
/// See more [on `type`][].
///
/// [on `type`]: https://wistia.com/support/developers/data-api#medias-response
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    Video,
    Audio,
    Image,
    PdfDocument,
    MicrosoftOfficeDocument,
    Swf,
    UnknownType,
}
