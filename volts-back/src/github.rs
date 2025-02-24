use anyhow::Result;
use oauth2::AccessToken;
use reqwest::{header, Client};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub id: i32,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Clone)]
pub struct GithubClient {
    base_url: String,
    client: Client,
}

impl Default for GithubClient {
    fn default() -> Self {
        GithubClient::new()
    }
}

impl GithubClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self {
            base_url: "https://api.github.com".to_string(),
            client,
        }
    }

    /// Does all the nonsense for sending a GET to Github.
    pub async fn request<T>(&self, url: &str, auth: &AccessToken) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, url);

        let result = self
            .client()
            .get(&url)
            .header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::AUTHORIZATION, format!("token {}", auth.secret()))
            .header(header::USER_AGENT, "crates.io (https://crates.io)")
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    /// Returns a client for making HTTP requests to upload crate files.
    ///
    /// The client will go through a proxy if the application was configured via
    /// `TestApp::with_proxy()`.
    ///
    /// # Panics
    ///
    /// Panics if the application was not initialized with a client.  This should only occur in
    /// tests that were not properly initialized.
    fn client(&self) -> &Client {
        &self.client
    }

    pub async fn current_user(&self, auth: &AccessToken) -> Result<GithubUser> {
        self.request("/user", auth).await
    }
}
