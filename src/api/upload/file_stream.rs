use crate::api::client::UploadClient;
use crate::constants::DEFAULT_FILENAME;
use crate::models::*;
use crate::types::Result;

use std::io::Read;

use hyper::Request;
use hyper_multipart::client::multipart::Form;
use hyper_multipart_rfc7578 as hyper_multipart;
use hyper_multipart_rfc7578::client::multipart::Body;

/// Client implementation to upload *streams* (file-like objects) and
/// *videos* via the Wistia **[Upload API]**.
///
/// Also check out the [`rust-wistia`] docs for usage and examples.
///
/// [`rust-wistia`]: https://docs.rs/rust-wistia
/// [Upload API]: https://wistia.com/support/developers/upload-api
///
#[derive(Clone)]
pub struct StreamUploader<'a, R: 'static + Read + Send + Sync, B = Body> {
    client: UploadClient<B>,
    req: UploadStreamRequest<'a>,
    reader: Option<R>,
}

impl<'a, R: 'static + Read + Send + Sync> StreamUploader<'a, R> {
    pub fn new(stream: R) -> Result<Self> {
        Self::new_with_stream_and_filename(stream, DEFAULT_FILENAME)
    }

    /// Create a `SteamUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the media file.
    ///
    pub fn new_with_stream_and_filename(stream: R, file_name: &'a str) -> Result<Self> {
        Ok(Self {
            client: UploadClient::from_env()?,
            req: UploadStreamRequest::new(file_name),
            reader: Some(stream),
        })
    }

    /// Create a `SteamUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the media file.
    ///
    pub fn new_with_filename(file_name: &'a str) -> Result<Self> {
        Ok(Self {
            client: UploadClient::from_env()?,
            req: UploadStreamRequest::new(file_name),
            reader: None,
        })
    }

    /// Create a `SteamUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `access_token` - An API access token used to make requests to the
    /// Wistia API.
    ///
    pub fn with_token(access_token: &str) -> Self {
        Self {
            client: UploadClient::from_token(access_token),
            req: UploadStreamRequest::new(DEFAULT_FILENAME),
            reader: None,
        }
    }

    /// Create a `SteamUploader` with a file path and an HTTPS client.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTPS client (UploadClient) to use for requests.
    /// Note that the client must support multipart form requests, via a
    /// `multipart::Body`.
    ///
    pub fn with_client(client: UploadClient<Body>) -> Self {
        Self {
            client,
            req: UploadStreamRequest::new(DEFAULT_FILENAME),
            reader: None,
        }
    }

    /// Sets the *reader stream* which will be used to upload to Wistia.
    pub fn stream(mut self, stream: R) -> Self {
        self.reader = Some(stream);
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

    /// Send the Upload File request (with the *multi-part form* data) to the
    /// Wistia [Upload API].
    ///
    /// [Upload API]: https://wistia.com/support/developers/upload-api
    ///
    // noinspection DuplicatedCode
    pub async fn send(self) -> Result<UploadResponse> {
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

        // TODO worth checking for unwrap()?
        form.add_reader_file("file", self.reader.unwrap(), self.req.file_name);

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
