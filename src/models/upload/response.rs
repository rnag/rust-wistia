use crate::models::{MediaStatus, MediaType, Thumbnail};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub created: String,
    pub updated: String,
    /// Note: this field will not be populated when uploading URL links
    #[serde(default)]
    pub duration: f64,
    pub hashed_id: String,
    #[serde(deserialize_with = "empty_string_is_none")]
    pub description: Option<String>,
    pub progress: f32,
    #[serde(default)]
    pub status: MediaStatus,
    pub thumbnail: Thumbnail,
    pub account_id: u64,
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::log::debug;
    use serde_json::from_str;

    /// Using the provided [Example Response][]
    ///
    /// [Example Response]: https://wistia.com/support/developers/upload-api#example-response
    #[test]
    fn test_deserialize_upload_response_works() {
        let resp = r#"
        {
          "id": 2208087,
          "account_id": 123456789,
          "name": "dramatic_squirrel.mp4",
          "description": "",
          "type": "Video",
          "created": "2012-10-26T16:47:09+00:00",
          "updated": "2012-10-26T16:47:10+00:00",
          "duration": 5.333000183105469,
          "hashed_id": "gn69c10tqw",
          "progress": 0.0,
          "thumbnail":
          {
            "url": "http://embed.wistia.com/deliveries/ffbada01610466e66f67a5dbbf473ed6574a6405.jpg?image_crop_resized=100x60",
            "width": 100,
            "height": 60
          }
        }
        "#;

        let resp: UploadResponse = from_str(resp).unwrap();

        debug!("{:#?}", resp);

        assert_eq!(resp.name, "dramatic_squirrel.mp4");
        assert_eq!(resp.description, None);
        assert_eq!(resp.status, MediaStatus::Queued);
        assert_eq!(resp.media_type, MediaType::Video);
    }
}
