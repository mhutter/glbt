use std::{fmt, rc::Rc};

use gloo_net::http::{Request, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use web_sys::RequestRedirect;

use crate::APP;

mod models;
pub use models::*;

pub type FetchResult<T> = Result<T, FetchError>;

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
        self.send(self.get("user")).await
    }

    /// Fetch up to 200 open non-draft merge requests.
    pub async fn get_open_mrs(&self) -> Result<Vec<MergeRequest>, FetchError> {
        self.send(self.get("merge_requests").query([
            ("per_page", "200"),
            ("scope", "all"),
            ("state", "opened"),
            ("wip", "no"),
        ]))
        .await
    }

    /// Get a single merge request
    pub async fn get_mr(&self, mr: &MergeRequest) -> FetchResult<MergeRequest> {
        self.send(self.get(&format!(
            "projects/{}/merge_requests/{}",
            mr.project_id, mr.iid
        )))
        .await
    }

    /// Fetch the pipelines for the latest commit of the given MR.
    pub async fn get_latest_mr_pipelines(&self, mr: &MergeRequest) -> FetchResult<Vec<Pipeline>> {
        self.send(self.get(&format!(
            "projects/{}/merge_requests/{}/pipelines",
            mr.project_id, mr.iid
        )))
        .await
        // Filter for Pipelines that match the current commit SHA
        .map(|vec: Vec<Pipeline>| vec.into_iter().filter(|p| p.sha == mr.sha).collect())
    }

    /// Update (aka "close" or "reopen") a merge request
    pub async fn update_mr(
        &self,
        mr: &MergeRequest,
        state: &StateEvent,
    ) -> FetchResult<MergeRequest> {
        self.send(
            self.put(&format!(
                "projects/{}/merge_requests/{}",
                mr.project_id, mr.iid
            ))
            // State to put in
            .query([("state_event", state.as_str())]),
        )
        .await
    }

    /// Merge a merge request
    pub async fn merge_mr(&self, mr: &MergeRequest) -> FetchResult<MergeRequest> {
        self.send(
            self.put(&format!(
                "projects/{}/merge_requests/{}/merge",
                mr.project_id, mr.iid
            ))
            .query([
                ("sha", mr.sha.as_str()),
                ("should_remove_source_branch", "true"),
            ]),
        )
        .await
    }

    /// Fetch a generic resource from the API.
    ///
    /// This method will set some common options (user agent header, authentication, redirect
    /// policy, ...), execute the request, and deserialize the response.
    ///
    /// You likely want to use one of the convienience methods provided by [`Gitlab`] instead.
    pub async fn send<T>(&self, req: RequestBuilder) -> FetchResult<T>
    where
        T: DeserializeOwned,
    {
        let res = req
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

    /// Create a new GET request with the given path
    pub fn get(&self, path: &str) -> RequestBuilder {
        Request::get(self.url(path).as_str())
    }
    /// Create a new PUT request with the given path
    pub fn put(&self, path: &str) -> RequestBuilder {
        Request::put(self.url(path).as_str())
    }

    pub fn url(&self, path: &str) -> Url {
        self.url.join(path).unwrap()
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

#[derive(Clone, Copy)]
pub enum StateEvent {
    Close,
    Reopen,
}
impl StateEvent {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Close => "close",
            Self::Reopen => "reopen",
        }
    }
}
