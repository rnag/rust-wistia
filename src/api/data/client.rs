use crate::auth::auth_token;
use crate::constants::ENV_VAR_NAME;
use crate::https::{get_https_client, tls};
use crate::models::Media;
use crate::status::raise_for_status;
use crate::utils::into_struct_from_slice;
use crate::RustWistiaError;

use std::borrow::Cow;
use std::time::Instant;

use hyper::client::{Client, HttpConnector};
use hyper::header::AUTHORIZATION;
use hyper::{Body, Method, Request};
use log::debug;
use serde::de::DeserializeOwned;

pub type WistiaClient<'a> = DataClient<'a>;

pub struct DataClient<'a> {
    /// Represents the [API access token] used to authenticate requests to the
    /// [Wistia API].
    ///
    /// [API access token]: https://wistia.com/support/developers/data-api#getting-started
    /// [Wistia API]: https://wistia.com/support/developers/upload-api
    pub access_token: Cow<'a, str>,
    /// The HTTPS client to use for sending requests.
    pub client: Client<tls::HttpsConnector<HttpConnector>>,
}

impl<'a> From<Cow<'a, str>> for DataClient<'a> {
    /// Create a new Wistia `DataClient` from an access token
    fn from(access_token: Cow<'a, str>) -> Self {
        let token = auth_token(&access_token);

        Self {
            access_token: Cow::Owned(token),
            client: get_https_client(),
        }
    }
}

impl<'a> From<&'a str> for DataClient<'a> {
    /// Create a new Wistia `DataClient` from an access token
    fn from(access_token: &'a str) -> Self {
        Self::from(Cow::Borrowed(access_token))
    }
}

impl<'a> From<String> for DataClient<'a> {
    /// Create a new Wistia `DataClient` from an access token
    fn from(access_token: String) -> Self {
        Self::from(Cow::Owned(access_token))
    }
}

impl<'a> DataClient<'a> {
    /// Create a new Wistia `DataClient` from an access token
    pub fn new(access_token: &'a str) -> Self {
        Self::from(access_token)
    }

    /// Initialize a new Wistia `DataClient` object from an API access token,
    /// assuming this is currently set in the environment.
    pub fn from_env() -> crate::Result<Self> {
        let token: String =
            std::env::var(ENV_VAR_NAME).map_err(|_| RustWistiaError::EnvVarNotFound {
                name: ENV_VAR_NAME.to_owned(),
            })?;

        Ok(Self::from(token))
    }

    /// Retrieve info on a media on Wistia (typically a video)
    pub async fn get_media(&self, video_id: &'a str) -> crate::Result<Media> {
        let url = format!(
            "https://api.wistia.com/v1/medias/{media_id}.json",
            media_id = video_id
        );

        self.get(&url).await
    }

    /// Make a GET request to the Wistia Data API
    pub async fn get<R: DeserializeOwned>(&'a self, url: &'a str) -> crate::Result<R> {
        let token = self.access_token.as_ref();

        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header(AUTHORIZATION, token)
            .body(Body::empty())?;

        self.make_request(url, req).await
    }

    /// Send the request to the Wistia Data API
    pub(crate) async fn make_request<R: DeserializeOwned>(
        &'a self,
        url: &'a str,
        req: Request<Body>,
    ) -> crate::Result<R> {
        let start = Instant::now();
        let mut resp = self.client.request(req).await?;
        debug!("Call Data API completed {:.2?}", start.elapsed());

        raise_for_status(url, &mut resp).await?;

        into_struct_from_slice(resp).await
    }
}
