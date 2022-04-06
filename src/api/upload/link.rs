use crate::api::client::UploadClient;
use crate::constants::UPLOAD_API;
use crate::models::*;
use crate::types::Result;

use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request};
use serde_urlencoded::to_string;

/// Client implementation to upload *URL links* to media files via the
/// Wistia **[Upload API]**.
///
/// Also check out the [`rust-wistia`] docs for usage and examples.
///
/// [`rust-wistia`]: https://docs.rs/rust-wistia
/// [Upload API]: https://wistia.com/support/developers/upload-api
///
#[derive(Clone)]
pub struct UrlUploader<'a, B = Body> {
    client: UploadClient<B>,
    req: UploadUrlRequest<'a>,
}

impl<'a> From<String> for UrlUploader<'a> {
    /// Create a new `UrlUploader` from an access token
    fn from(token: String) -> Self {
        Self {
            client: UploadClient::from(token),
            req: UploadUrlRequest::default(),
        }
    }
}

impl<'a> From<&str> for UrlUploader<'a> {
    /// Create a new `UrlUploader` from an access token
    fn from(token: &str) -> Self {
        Self {
            client: UploadClient::from(token),
            req: UploadUrlRequest::default(),
        }
    }
}

impl<'a> UrlUploader<'a> {
    /// Create an `UrlUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `url` - A publicly-accessible URL link to the media file. The link
    /// will be *form-url encoded* into the request body.
    ///
    pub fn new(url: &'a str) -> Result<Self> {
        Ok(Self {
            client: UploadClient::from_env()?,
            req: UploadUrlRequest {
                url,
                ..Default::default()
            },
        })
    }

    /// Create an `UrlUploader` with a new HTTPS client and a Wistia access
    /// token.
    ///
    /// # Arguments
    ///
    /// * `url` - A publicly-accessible URL link to the media file. The link
    /// will be *form-url encoded* into the request body.
    /// * `access_token` - An API access token used to make requests to the
    /// Wistia API.
    ///
    pub fn with_token(url: &'a str, access_token: &str) -> Self {
        Self {
            client: UploadClient::from(access_token),
            req: UploadUrlRequest {
                url,
                ..Default::default()
            },
        }
    }

    /// Create an `UrlUploader` with a URL link and an HTTPS client.
    ///
    /// # Arguments
    ///
    /// * `url` - A publicly-accessible URL link to the media file. The link
    /// will be *form-url encoded* into the request body.
    /// * `client` - The HTTPS client (UploadClient) to use for requests.
    ///
    pub fn with_client(url: &'a str, client: UploadClient<Body>) -> Self {
        Self {
            client,
            req: UploadUrlRequest {
                url,
                ..Default::default()
            },
        }
    }

    /// Set the publicly-accessible URL link to the media file. The link
    /// will be *form-url encoded* into the request body.
    ///
    /// # Note
    /// This method call is only needed when the `UrlUploader::from`
    /// constructor is called.
    pub fn url(mut self, url: &'a str) -> Self {
        self.req.url = url;
        self
    }

    /// The hashed id of the project to upload media into. If omitted, a new
    /// project will be created and uploaded to. The naming convention used
    /// for such projects is `Uploads_YYYY-MM-DD`.
    pub fn project_id(mut self, project_id: &'a str) -> Self {
        self.req.project_id = Some(project_id);
        self
    }

    /// A display name to use for the media in Wistia. If omitted, the filename
    /// will be used instead. This field is limited to 255 characters.
    pub fn name(mut self, name: &'a str) -> Self {
        self.req.name = Some(name);
        self
    }

    /// A description to use for the media in Wistia. You can use basic HTML
    /// here, but note that both HTML and CSS will be sanitized.
    pub fn description(mut self, description: &'a str) -> Self {
        self.req.description = Some(description);
        self
    }

    /// A Wistia contact id, an integer value. If omitted, it will default to
    /// the contact_id of the account’s owner.
    pub fn contact_id(mut self, contact_id: &'a str) -> Self {
        self.req.contact_id = Some(contact_id);
        self
    }

    /// Send the Upload URL request (with the *form-url encoded* data) to the
    /// Wistia [Upload API].
    ///
    /// [Upload API]: https://wistia.com/support/developers/upload-api
    ///
    pub async fn send(&self) -> Result<UploadResponse> {
        // Build the query parameters to pass to the Upload API

        let params = UploadRequest {
            access_token: self.client.access_token.as_str(),
            url: Some(self.req.url),
            project_id: self.req.project_id,
            name: self.req.name,
            description: self.req.description,
            contact_id: self.req.contact_id,
        };

        let url_encoded_data = to_string(params)?;

        // Create a request instance
        let req = Request::post(UPLOAD_API)
            // Make sure the server knows the data is `x-www-form-urlencoded`
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(Body::from(url_encoded_data))?;

        // Send the request
        self.client.make_request(UPLOAD_API, req).await
    }
}
