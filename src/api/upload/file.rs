use crate::api::client::UploadClient;
use crate::models::*;
use crate::types::Result;
use crate::RustWistiaError;

use std::fmt::Debug;
use std::io;
use std::path::Path;

use hyper::Request;
use hyper_multipart::client::multipart::Form;
use hyper_multipart_rfc7578 as hyper_multipart;
use hyper_multipart_rfc7578::client::multipart::Body;

/// Client implementation to upload *files* and *videos* via the Wistia
/// **[Upload API]**.
///
/// Also check out the [`rust-wistia`] docs for usage and examples.
///
/// [`rust-wistia`]: https://docs.rs/rust-wistia
/// [Upload API]: https://wistia.com/support/developers/upload-api
///
pub struct FileUploader<'a, P> {
    client: UploadClient<Body>,
    req: UploadFileRequest<'a, P>,
}

impl<'a, P> FileUploader<'a, P>
where
    P: AsRef<Path> + Debug,
{
    /// Create a `FileUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the media file. The contents of this file
    /// will be multipart-form encoded into the request body.
    ///
    pub fn new(file_path: P) -> Result<Self> {
        Ok(Self {
            client: UploadClient::from_env()?,
            req: UploadFileRequest::new(file_path),
        })
    }

    /// Create a `FileUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the media file. The contents of this file
    /// will be multipart-form encoded into the request body.
    /// * `access_token` - An API access token used to make requests to the
    /// Wistia API.
    ///
    pub fn with_token(file_path: P, access_token: &'static str) -> Self {
        Self {
            client: UploadClient::from_token(access_token),
            req: UploadFileRequest::new(file_path),
        }
    }

    /// Create a `FileUploader` with a file path and an HTTPS client.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the media file. The contents of this file
    /// will be multipart-form encoded into the request body.
    /// * `client` - The HTTPS client (UploadClient) to use for requests.
    /// Note that the client must support multipart form requests, via a
    /// `multipart::Body`.
    ///
    pub fn with_client(file_path: P, client: UploadClient<Body>) -> Self {
        Self {
            client,
            req: UploadFileRequest::new(file_path),
        }
    }

    /// The hashed id of the project to upload media into. If omitted, a new
    /// project will be created and uploaded to. The naming convention used
    /// for such projects is `Uploads_YYYY-MM-DD`.
    pub fn project_id(mut self, project_id: &'a str) -> FileUploader<'a, P> {
        self.req.project_id = Some(project_id);
        self
    }

    /// A display name to use for the media in Wistia. If omitted, the filename
    /// will be used instead. This field is limited to 255 characters.
    pub fn name(mut self, name: &'a str) -> FileUploader<'a, P> {
        self.req.name = Some(name);
        self
    }

    /// A description to use for the media in Wistia. You can use basic HTML
    /// here, but note that both HTML and CSS will be sanitized.
    pub fn description(mut self, description: &'a str) -> FileUploader<'a, P> {
        self.req.description = Some(description);
        self
    }

    /// A Wistia contact id, an integer value. If omitted, it will default to
    /// the contact_id of the account’s owner.
    pub fn contact_id(mut self, contact_id: &'a str) -> FileUploader<'a, P> {
        self.req.contact_id = Some(contact_id);
        self
    }

    /// Send the Upload File request (with the *multi-part form* data) to the
    /// Wistia [Upload API].
    ///
    /// [Upload API]: https://wistia.com/support/developers/upload-api
    ///
    pub async fn send(&self) -> Result<UploadResponse> {
        // Build the query parameters to pass to the Upload API

        let params = UploadRequest {
            access_token: self.client.access_token.as_str(),
            url: None,
            project_id: self.req.project_id,
            name: self.req.name,
            description: None,
            contact_id: self.req.contact_id,
        };

        let url = UploadClient::<Body>::build_url(params)?;

        // Create a request instance and multipart form
        let req_builder = Request::post(&url);
        let mut form = Form::default();

        // Add multi-part form fields

        form.add_file("file", &self.req.file_path)
            .map_err(|e: io::Error| match e.kind() {
                io::ErrorKind::NotFound => RustWistiaError::FileNotFound(
                    self.req.file_path.as_ref().to_string_lossy().to_string(),
                ),
                _ => RustWistiaError::Io(e),
            })?;

        if let Some(description) = self.req.description {
            form.add_text("description", description);
        }

        // Update a request instance with the multipart Content-Type header
        // and the payload data.
        let form = form.set_body::<Body>(req_builder).unwrap();

        // Send the request
        self.client.make_request(&url, form).await
    }
}
