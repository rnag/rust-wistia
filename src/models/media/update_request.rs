use serde::Serialize;

/// Represents a [Medias: Update] request.
///
/// [Medias: Update]: https://wistia.com/support/developers/data-api#the-request-7
///
#[derive(Default, Debug, PartialEq, Serialize)]
pub struct UpdateMediaRequest {
    /// The hashed Video Id (example: `abc1234567`).
    #[serde(skip_serializing)]
    pub id: String,
    /// The media's new name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Wistia hashed ID of an image that will replace the still that’s
    /// displayed before the player starts playing. Will return failure message
    /// unless media to update is a video, and new still is an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_still_media_id: Option<String>,
    /// A new description for this media. Accepts plain text or markdown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateMediaRequest {
    /// Sets the media's new name
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    /// Sets the Wistia hashed ID of an image that will replace the still that’s
    /// displayed before the player starts playing.
    pub fn new_still_media_id(mut self, new_still_media_id: &str) -> Self {
        self.new_still_media_id = Some(new_still_media_id.to_owned());
        self
    }

    /// Sets a new description for this media. Accepts plain text or markdown.
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
}
