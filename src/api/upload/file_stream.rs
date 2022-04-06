use crate::api::client::UploadClient;
use crate::constants::DEFAULT_FILENAME;
use crate::https::tls;
use crate::models::*;
use crate::types::Result;
use crate::utils::{stream_reader_from_url, stream_reader_from_url_and_arc_client};

use std::io::{Cursor, Read};
use std::sync::Arc;

use hyper::client::HttpConnector;
use hyper::{body::Bytes, Client, Request};
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

impl<'a> StreamUploader<'a, Cursor<Bytes>> {
    /// Create a new `StreamUploader` which uses the *bytes* content downloaded
    /// from a publicly accessible **url**.
    ///
    /// # Arguments
    ///
    /// * `url` - A public accessible url to the media which will be downloaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::{StreamUploader, https::get_https_client};
    ///
    /// let mut uploader = StreamUploader::with_url("https://google.com/my/image").await?;
    /// ```
    pub async fn with_url(url: &str) -> Result<StreamUploader<'a, Cursor<Bytes>>> {
        let stream = stream_reader_from_url(url, None).await?;
        Self::new(stream)
    }

    /// Create a new `StreamUploader` which uses the *bytes* content downloaded
    /// from a publicly accessible **url**.
    ///
    /// # Arguments
    ///
    /// * `url` - A public accessible url to the media which will be downloaded.
    /// * `access_token` - An API access token used to make requests to the
    /// Wistia API.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::{StreamUploader, https::get_https_client};
    ///
    /// let mut uploader = StreamUploader::with_url_and_token(
    ///     "https://google.com/my/image",
    ///     "my-token"
    /// )
    /// .await?;
    /// ```
    pub async fn with_url_and_token(
        url: &str,
        access_token: &str,
    ) -> Result<StreamUploader<'a, Cursor<Bytes>>> {
        let stream = stream_reader_from_url(url, None).await?;
        Ok(Self::with_token(access_token).stream(stream))
    }

    /// Create a new `StreamUploader` which uses the *bytes* content downloaded
    /// from a publicly accessible **url**.
    ///
    /// # Arguments
    ///
    /// * `url` - A public accessible url to the media which will be downloaded.
    /// * `client` - An optional HTTPS client to use for downloading the media.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::{StreamUploader, https::get_https_client};
    ///
    /// let client = get_https_client();
    /// let mut uploader = StreamUploader::with_url_and_client("https://google.com/my/image", client).await?;
    /// ```
    pub async fn with_url_and_client(
        url: &str,
        client: impl Into<Option<Client<tls::HttpsConnector<HttpConnector>>>>,
    ) -> Result<StreamUploader<'a, Cursor<Bytes>>> {
        let stream = stream_reader_from_url(url, client).await?;
        Self::new(stream)
    }

    /// Create a new `StreamUploader` which uses the *bytes* content downloaded
    /// from a publicly accessible **url**.
    ///
    /// # Arguments
    ///
    /// * `url` - A public accessible url to the media which will be downloaded.
    /// * `client` - An optional HTTPS client to use for downloading the media.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use rust_wistia::{StreamUploader, https::get_https_client};
    ///
    /// let client = Arc::new(get_https_client());
    /// let mut uploader = StreamUploader::with_url_and_arc_client("https://google.com/my/image", client).await?;
    /// ```
    pub async fn with_url_and_arc_client(
        url: &str,
        client: Arc<Client<tls::HttpsConnector<HttpConnector>>>,
    ) -> Result<StreamUploader<'a, Cursor<Bytes>>> {
        let stream = stream_reader_from_url_and_arc_client(url, client).await?;
        Self::new(stream)
    }
}

impl<'a, R: 'static + Read + Send + Sync> StreamUploader<'a, R> {
    /// Create a `StreamUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `stream` - A readable file-like *stream* object to upload.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::StreamUploader;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = StreamUploader::new(bytes)?;
    ///
    /// let res = uploader.name("My Video Name").send()?.await?;
    /// ```
    ///    
    pub fn new(stream: R) -> Result<Self> {
        Self::with_stream_and_filename(stream, DEFAULT_FILENAME)
    }

    /// Create a `SteamUploader` with a new HTTPS client, with the access token
    /// retrieved from the environment.
    ///
    /// # Arguments
    ///
    /// * `stream` - A readable file-like *stream* object to upload.
    /// * `file_name` - The name of the media file.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::StreamUploader;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = StreamUploader::with_stream_and_filename(bytes, "my_file.mp4")?;
    ///
    /// let res = uploader.send()?.await?;
    /// ```
    ///
    pub fn with_stream_and_filename(stream: R, file_name: &'a str) -> Result<Self> {
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
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::StreamUploader;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = StreamUploader::with_filename("my_file.mp4")?.stream(bytes);
    ///
    /// let res = uploader.send()?.await?;
    /// ```
    ///
    pub fn with_filename(file_name: &'a str) -> Result<Self> {
        Ok(Self {
            client: UploadClient::from_env()?,
            req: UploadStreamRequest::new(file_name),
            reader: None,
        })
    }

    /// Create a `SteamUploader` with a new HTTPS client and a Wistia access
    /// token.
    ///
    /// # Arguments
    ///
    /// * `access_token` - An API access token used to make requests to the
    /// Wistia API.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::StreamUploader;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = StreamUploader::with_token("my-token").stream(bytes);
    ///
    /// let res = uploader.send()?.await?;
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use rust_wistia::{StreamUploader, UploadClient};
    /// use std::io::Cursor;
    ///
    /// let client = UploadClient::from_env()?;
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = StreamUploader::with_client(client).stream(bytes);
    ///
    /// let res = uploader.send()?.await?;
    /// ```
    ///
    pub fn with_client(client: UploadClient<Body>) -> Self {
        Self {
            client,
            req: UploadStreamRequest::new(DEFAULT_FILENAME),
            reader: None,
        }
    }

    /// Sets the *reader stream* which will be used to upload to Wistia.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let uploader = rust_wistia::StreamUploader::with_filename("my_file.mp4")?.stream(bytes);
    /// ```
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
