use serde::Serialize;

/// Model representing a [Request] to the Wistia [Upload API].
///
/// # Note
/// This only includes the parameters which may be encoded into the part
/// of the query string.
///
/// The only exceptions are `description`, which is expected to be long and
/// which may be more performant to be multipart-form encoded, and the `file`
/// parameter which *must* be multipart-form encoded into the request body.
///
/// [Upload API]: https://wistia.com/support/developers/upload-api
/// [Request]: https://wistia.com/support/developers/upload-api#the-request
///
#[derive(Default, Serialize)]
pub struct UploadRequest<'a> {
    /// **Required**. A 64 character hex string. This parameter can be found
    /// on your [API access page].
    ///
    /// [API access page]: https://wistia.com/support/developers/data-api#getting-started
    pub access_token: &'a str,
    /// **Required** unless `file` is specified. The web location of the media
    /// file to import.
    pub url: Option<&'a str>,
    /// The hashed id of the project to upload media into. If omitted, a new
    /// project will be created and uploaded to. The naming convention used
    /// for such projects is `Uploads_YYYY-MM-DD`.
    pub project_id: Option<&'a str>,
    /// A display name to use for the media in Wistia. If omitted, the filename
    /// will be used instead. This field is limited to 255 characters.
    pub name: Option<&'a str>,
    /// A description to use for the media in Wistia. You can use basic HTML
    /// here, but note that both HTML and CSS will be sanitized.
    pub description: Option<&'a str>,
    /// A Wistia contact id, an integer value. If omitted, it will default to
    /// the contact_id of the account’s owner.
    pub contact_id: Option<&'a str>,
}

#[derive(Default)]
pub(crate) struct UploadUrlRequest<'a> {
    /// **Required**. The web location of the media file to import.
    pub url: &'a str,
    /// The hashed id of the project to upload media into. If omitted, a new
    /// project will be created and uploaded to. The naming convention used
    /// for such projects is `Uploads_YYYY-MM-DD`.
    pub project_id: Option<&'a str>,
    /// A display name to use for the media in Wistia. If omitted, the filename
    /// will be used instead. This field is limited to 255 characters.
    pub name: Option<&'a str>,
    /// A description to use for the media in Wistia. You can use basic HTML
    /// here, but note that both HTML and CSS will be sanitized.
    pub description: Option<&'a str>,
    /// A Wistia contact id, an integer value. If omitted, it will default to
    /// the contact_id of the account’s owner.
    pub contact_id: Option<&'a str>,
}

// We get a warning in the `examples/` that this is not used, but it *will*
// be used when the optional feature is enabled.
#[allow(unused)]
pub(crate) struct UploadFileRequest<'a, P> {
    /// **Required**. The path to the media file. The contents of this file
    /// will be multipart-form encoded into the request body.
    pub file_path: P,
    /// The hashed id of the project to upload media into. If omitted, a new
    /// project will be created and uploaded to. The naming convention used
    /// for such projects is `Uploads_YYYY-MM-DD`.
    pub project_id: Option<&'a str>,
    /// A display name to use for the media in Wistia. If omitted, the filename
    /// will be used instead. This field is limited to 255 characters.
    pub name: Option<&'a str>,
    /// A description to use for the media in Wistia. You can use basic HTML
    /// here, but note that both HTML and CSS will be sanitized.
    pub description: Option<&'a str>,
    /// A Wistia contact id, an integer value. If omitted, it will default to
    /// the contact_id of the account’s owner.
    pub contact_id: Option<&'a str>,
}

impl<'a, P> UploadFileRequest<'a, P> {
    #[allow(unused)]
    pub(crate) fn new(file_path: P) -> Self {
        Self {
            file_path,
            project_id: None,
            name: None,
            description: None,
            contact_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_urlencoded::to_string;

    #[test]
    fn test_it_works() {
        let req = UploadRequest {
            access_token: "abc123",
            url: None,
            project_id: None,
            name: None,
            description: None,
            contact_id: None,
        };

        assert_eq!(to_string(req).unwrap(), "access_token=abc123");
    }

    #[test]
    fn test_it_works_with_other_params() {
        let req = UploadRequest {
            access_token: "xyz123",
            url: Some("https://test-url.com/my/path?key=\"hello world!@#$?%&\"&value="),
            project_id: Some("abc321"),
            name: None,
            description: None,
            contact_id: Some("my contact <abc@xyz.org>"),
        };

        assert_eq!(to_string(req).unwrap(), "access_token=xyz123&url=https%3A%2F%2Ftest-url.com%2Fmy%2Fpath%3Fkey%3D%22hello+world%21%40%23%24%3F%25%26%22%26value%3D&project_id=abc321&contact_id=my+contact+%3Cabc%40xyz.org%3E");
    }
}
