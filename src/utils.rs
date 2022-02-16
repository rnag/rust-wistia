//! Library-specific utilities, mainly for internal use.
//!
use crate::Result;

use std::io::{BufReader, Read};

use hyper::body::Buf;
use hyper::{Body, Response, Uri};
use serde::de;

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
