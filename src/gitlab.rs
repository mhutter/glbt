use std::{fmt, rc::Rc};

use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use web_sys::RequestRedirect;

use crate::APP;

pub type ID = i32;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gitlab {
    #[serde(rename = "u")]
    url: Url,
    #[serde(rename = "t")]
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
        self.get("user", []).await
    }

    pub async fn get_open_mrs(&self) -> Result<Vec<MergeRequest>, FetchError> {
        self.get(
            "merge_requests",
            [
                ("per_page", "200"),
                ("scope", "all"),
                ("state", "opened"),
                ("wip", "no"),
            ],
        )
        .await
    }

    /// Fetch a generic resource from the API.
    ///
    /// You likely want to use one of the convienience methods provided by [`Gitlab`] instead.
    pub async fn get<'a, T, Q>(&self, path: &str, query: Q) -> Result<T, FetchError>
    where
        T: DeserializeOwned,
        Q: IntoIterator<Item = (&'static str, &'static str)>,
    {
        let res = Request::get(self.url.join(path).unwrap().as_str())
            .query(query)
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

#[derive(Clone, Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Failed to send request: {0}")]
    RequestFailed(#[source] Rc<gloo_net::Error>),

    #[error("HTTP {status}: {body}")]
    HttpStatus { status: u16, body: String },

    #[error("Failed to deserialize Body: {0}")]
    Json(#[source] Rc<gloo_net::Error>),
}

impl FetchError {
    fn request(err: gloo_net::Error) -> Self {
        Self::RequestFailed(Rc::new(err))
    }
    fn json(err: gloo_net::Error) -> Self {
        Self::Json(Rc::new(err))
    }
}

impl fmt::Display for Gitlab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.url.host_str().expect("host").fmt(f)
    }
}

#[derive(Clone, Deserialize)]
pub struct MergeRequest {
    pub iid: ID,
    pub id: ID,
    pub project_id: ID,
    pub title: String,
    pub references: References,
    pub sha: String,
    pub web_url: String,
}

#[derive(Clone, Deserialize)]
pub struct References {
    pub full: String,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
}
