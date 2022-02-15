use crate::constants::{ENV_VAR_NAME, UPLOAD_API};
use crate::https::{get_https_client, tls};
use crate::log::debug;
use crate::models::*;
use crate::status::raise_for_status;
use crate::types::Result;
use crate::utils::into_struct_from_slice;

use std::env::var;
use std::io::ErrorKind;
use std::time::Instant;

use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request};
use serde_urlencoded::to_string;

/// Client used to make requests  to the Wistia **[Upload API]**.
///
/// # Note
/// Prefer to use this with one of the concrete implementations, i.e.
/// [`rust_wistia::FileUploader`] or [`rust_wistia::UrlUploader`].
///
/// Also check out the [`rust-wistia`] docs for usage and examples.
///
/// [`rust-wistia`]: https://docs.rs/rust-wistia
/// [Upload API]: https://wistia.com/support/developers/upload-api
///
pub struct UploadClient<B = Body> {
    /// Represents the [API access token] used to authenticate requests to the
    /// [Wistia API].
    ///
    /// [API access token]: https://wistia.com/support/developers/data-api#getting-started
    /// [Wistia API]: https://wistia.com/support/developers/upload-api
    pub access_token: String,
    /// The HTTPS client to use for sending requests.
    client: Client<tls::HttpsConnector<HttpConnector>, B>,
}

impl<B: HttpBody + Send + 'static> UploadClient<B>
where
    <B as HttpBody>::Data: Send,
    <B as HttpBody>::Error: Into<Box<(dyn std::error::Error + Send + Sync + 'static)>>,
{
    /// Initialize a new `UploadClient` object from an [API access token],
    /// assuming this is currently set in the environment.
    ///
    /// [API access token]: https://wistia.com/support/developers/data-api#getting-started
    pub fn from_env() -> Result<Self> {
        let token = match var(ENV_VAR_NAME) {
            Ok(val) => Ok(val),
            Err(_) => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "Environment variable `{name}` must be set.",
                    name = ENV_VAR_NAME
                ),
            )),
        }?;

        Ok(Self::new(&token))
    }

    /// Initialize a new `UploadClient` object from an [API access token].
    ///
    /// [API access token]: https://wistia.com/support/developers/data-api#getting-started
    pub fn from_token(token: &'static str) -> Self {
        Self {
            access_token: token.to_string(),
            client: get_https_client(),
        }
    }

    /// Constructor function, for internal use
    fn new(access_token: &str) -> Self {
        let client = get_https_client();

        Self {
            access_token: access_token.to_string(),
            client,
        }
    }

    /// Build the URL with the url-encoded *query parameters* included
    pub fn build_url(params: UploadRequest) -> Result<String> {
        let query = to_string(params)?;

        // Build the URL with the query parameters included
        let mut url = String::with_capacity(UPLOAD_API.len() + 1 + query.len());
        url.push_str(UPLOAD_API);
        url.push('?');
        url.push_str(query.as_str());

        Ok(url)
    }

    /// Send the request to the Wistia Upload API
    pub async fn make_request<'a>(
        &'a self,
        url: &'a str,
        req: Request<B>,
    ) -> Result<UploadResponse> {
        let start = Instant::now();
        let mut resp = self.client.request(req).await?;
        debug!("Call Upload API completed {:.2?}", start.elapsed());

        raise_for_status(url, &mut resp).await?;

        into_struct_from_slice(resp).await
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_url_encoded_with_struct() {
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct Meal<'a> {
            bread: &'a str,
            cheese: &'a str,
            meat: &'a str,
            fat: &'a str,
        }

        let m = Meal {
            bread: "baguette",
            cheese: "comté",
            meat: "ham",
            fat: "butter",
        };

        assert_eq!(
            serde_urlencoded::to_string::<Meal>(m),
            Ok("bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter".to_owned())
        );
    }
}
