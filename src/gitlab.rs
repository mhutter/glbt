use std::fmt;

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use web_sys::RequestRedirect;

use crate::APP;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gitlab {
    #[serde(rename="u")]
    url: Url,
    #[serde(rename="t")]
    authorization: String,
}

impl Gitlab {
    pub fn new(url: &str, token: &str) -> Result<Self, NewGitlabError> {
        let url = Url::parse(url)?.join("api/v4/")?;
        let authorization = format!("Bearer {token}");
        Ok(Self { url, authorization })
    }

    /// Fetch the currently authenticated user
    pub async fn get_self(&self) -> Result<User, FetchError> {
        self.get("user").await
    }

    /// Fetch a generic resource from the API.
    ///
    /// You likely want to use one of the convienience methods provided by [`Gitlab`] instead.
    pub async fn get<T>(&self, path: &str) -> Result<T, FetchError>
    where
        T: DeserializeOwned,
    {
        let res = Request::get(self.url.join(path).unwrap().as_str())
            .header("User-Agent", APP)
            .header("Authorization", &self.authorization)
            .redirect(RequestRedirect::Error)
            .send()
            .await
            .map_err(FetchError::request)?;

        let status = res.status();
        if status > 299 {
            let body = res.text().await.unwrap_or_default();
            return Err(FetchError::HttpStatus { status, body });
        }

        let value = res.json().await.map_err(FetchError::json)?;

        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewGitlabError {
    #[error("URL is invalid: {0}")]
    UrlInvalid(#[from] url::ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Failed to send request: {0}")]
    RequestFailed(#[source] gloo_net::Error),

    #[error("HTTP {status}: {body}")]
    HttpStatus { status: u16, body: String },

    #[error("Failed to deserialize Body: {0}")]
    Json(#[source] gloo_net::Error),
}

impl FetchError {
    fn request(err: gloo_net::Error) -> Self {
        Self::RequestFailed(err)
    }
    fn json(err: gloo_net::Error) -> Self {
        Self::Json(err)
    }
}

impl fmt::Display for Gitlab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.url.host_str().expect("host").fmt(f)
    }
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
}
