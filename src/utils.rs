//! Library-specific utilities, mainly for internal use.
//!
use crate::{https::get_https_client, tls, Result};

use std::io::{BufReader, Cursor, Read};
use std::sync::Arc;

use hyper::{
    body::{Buf, Bytes},
    client::HttpConnector,
    Body, Client, Request, Response, Uri,
};
use serde::de;

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn host_with_path(url: &str) -> Result<String> {
    let uri: Uri = url.parse()?;
    let host = uri.host().unwrap();
    let path = uri.path();

    Ok(format!("{}{}", host, path))
}

pub async fn into_struct_from_slice<T>(resp: Response<Body>) -> Result<T>
where
    T: de::DeserializeOwned,
{
    // asynchronously concatenate the buffer from a body into bytes
    let bytes = hyper::body::to_bytes(resp).await?;

    // try to parse as json with serde_json
    Ok(serde_json::from_slice(&bytes)?)
}

/// Read the body content of a mutable reference to a `Response` object
/// into a string.
pub async fn resp_to_string(resp: &mut Response<Body>) -> Result<String> {
    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(resp).await?;

    // use a buffered reader
    let mut reader = BufReader::new(body.reader());

    // read BufReader contents into a string
    let mut body_string = String::new();
    reader.read_to_string(&mut body_string)?;

    Ok(body_string)
}

/// Create a new *stream reader* from the *bytes* content downloaded from a
/// publicly accessible **url**.
///
/// # Arguments
///
/// * `url` - A public accessible url to the media which will be downloaded.
/// * `client` - An optional HTTPS client to use for downloading the media.
///
/// # Examples
///
/// ```rust,ignore
/// use rust_wistia::https::get_https_client;
/// use rust_wistia::utils::stream_reader_from_url;
///
/// let client = get_https_client();
/// let stream = stream_reader_from_url("https://google.com/my/image", client).await?;
/// ```
pub async fn stream_reader_from_url(
    url: &str,
    client: impl Into<Option<Client<tls::HttpsConnector<HttpConnector>>>>,
) -> Result<Cursor<Bytes>> {
    // resolve HTTPS client
    let client = client.into().unwrap_or_else(get_https_client);

    // make a GET request to download the url contents
    let req = Request::get(url).body(Body::empty())?;
    let resp = client.request(req).await?;

    // get the bytes content from the url
    let (_, body) = resp.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;

    // create and return the reader
    Ok(Cursor::new(bytes))
}

/// Create a new *stream reader* from the *bytes* content downloaded from a
/// publicly accessible **url**.
///
/// # Arguments
///
/// * `url` - A public accessible url to the media which will be downloaded.
/// * `client` - An Arc HTTPS client to use for downloading the media.
///
/// # Examples
///
/// ```rust,ignore
/// use std::sync::Arc;
/// use rust_wistia::https::get_https_client;
/// use rust_wistia::utils::stream_reader_from_url_and_arc_client;
///
/// let client = Arc::new(get_https_client());
/// let stream = stream_reader_from_url_and_arc_client("https://google.com/my/image", client).await?;
/// ```
pub async fn stream_reader_from_url_and_arc_client(
    url: &str,
    client: Arc<Client<tls::HttpsConnector<HttpConnector>>>,
) -> Result<Cursor<Bytes>> {
    // make a GET request to download the url contents
    let req = Request::get(url).body(Body::empty())?;
    let resp = client.request(req).await?;

    // get the bytes content from the url
    let (_, body) = resp.into_parts();
    let bytes = hyper::body::to_bytes(body).await?;

    // create and return the reader
    Ok(Cursor::new(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_host_without_path() {
        let url = "https://google.com";
        let new_url = host_with_path(url).unwrap();

        assert_eq!(new_url, "google.com/");
    }

    #[test]
    fn test_host_and_path() {
        let url = "https://google.com/my/path?key=value&key2=value2";
        let new_url = host_with_path(url).unwrap();

        assert_eq!(new_url, "google.com/my/path");
    }
}
