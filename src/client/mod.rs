pub mod response;

mod oauth;

use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::header::RETRY_AFTER;

use crate::client::response::FcmResponse;
use crate::message::{Message, MessageWrapper};

use self::{oauth::OauthClient, response::RetryAfter};

pub use self::oauth::OauthError;

#[derive(thiserror::Error, Debug)]
pub enum FcmClientError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("OAuth error: {0}")]
    Oauth(OauthError),
    #[error("Dotenvy error: {0}")]
    Dotenvy(#[from] dotenvy::Error),
    #[error("Retry-After HTTP header value is not valid string")]
    RetryAfterHttpHeaderIsNotString,
    #[error("Retry-After HTTP header value is not valid, error: {error}, value: {value}")]
    RetryAfterHttpHeaderInvalid { error: chrono::ParseError, value: String },
}

impl FcmClientError {
    /// If this is `true` then most likely current service account
    /// key is invalid.
    pub fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool {
        match self {
            FcmClientError::Oauth(error) => error.is_access_token_missing_even_if_server_requests_completed(),
            _ => false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FcmClientBuilder {
    service_account_key_json_string: Option<String>,
    service_account_key_json_path: Option<PathBuf>,
    token_cache_json_path: Option<PathBuf>,
    fcm_request_timeout: Option<Duration>,
    dry_run: Option<bool>,
}

impl FcmClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set path to the service account key JSON file. Default is to use
    /// path from the `GOOGLE_APPLICATION_CREDENTIALS` environment variable
    /// (which can be also located in `.env` file).
    pub fn service_account_key_json_path(mut self, service_account_key_json_path: impl AsRef<Path>) -> Self {
        self.service_account_key_json_path = Some(service_account_key_json_path.as_ref().to_path_buf());
        self
    }

    /// Set timeout for FCM requests. Default is no timeout.
    ///
    /// If this is set the value should be at least 10 seconds as FCM
    /// docs have that value as the minimum timeout.
    /// <https://firebase.google.com/docs/cloud-messaging/scale-fcm#timeouts>
    pub fn fcm_request_timeout(mut self, fcm_request_timeout: Duration) -> Self {
        self.fcm_request_timeout = Some(fcm_request_timeout);
        self
    }

    /// Set path to the token cache JSON file. Default is no token cache JSON file.
    pub fn token_cache_json_path(mut self, token_cache_json_path: impl AsRef<Path>) -> Self {
        self.token_cache_json_path = Some(token_cache_json_path.as_ref().to_path_buf());
        self
    }

    /// Set service account key JSON. Default is to use
    /// path from the `GOOGLE_APPLICATION_CREDENTIALS` environment variable
    /// (which can be also located in `.env` file).
    ///
    /// This overrides `service_account_key_json_path`.
    pub fn service_account_key_json_string(mut self, service_account_key_json_string: impl Into<String>) -> Self {
        self.service_account_key_json_string = Some(service_account_key_json_string.into());
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
    }

    pub async fn build(self) -> Result<FcmClient, FcmClientError> {
        FcmClient::new_from_builder(self).await
    }
}

/// An async client for sending the notification payload.
pub struct FcmClient {
    http_client: reqwest::Client,
    oauth_client: OauthClient,
    pub dry_run: bool,
}

impl FcmClient {
    pub fn builder() -> FcmClientBuilder {
        FcmClientBuilder::new()
    }

    async fn new_from_builder(fcm_builder: FcmClientBuilder) -> Result<Self, FcmClientError> {
        let builder = reqwest::ClientBuilder::new();
        let builder = if let Some(timeout) = fcm_builder.fcm_request_timeout {
            builder.timeout(timeout)
        } else {
            builder
        };
        let http_client = builder.build()?;

        let oauth_client = if let Some(key_json) = fcm_builder.service_account_key_json_string {
            OauthClient::create_with_string_key(key_json, fcm_builder.token_cache_json_path)
                .await
                .map_err(FcmClientError::Oauth)?
        } else {
            let service_account_key_path = if let Some(path) = fcm_builder.service_account_key_json_path {
                path
            } else {
                dotenvy::var("GOOGLE_APPLICATION_CREDENTIALS")?.into()
            };

            OauthClient::create_with_key_file(service_account_key_path, fcm_builder.token_cache_json_path)
                .await
                .map_err(FcmClientError::Oauth)?
        };

        Ok(FcmClient {
            http_client,
            oauth_client,
            dry_run: fcm_builder.dry_run.unwrap_or(false),
        })
    }

    pub async fn send(&self, message: impl AsRef<Message>) -> Result<FcmResponse, FcmClientError> {
        let access_token = self
            .oauth_client
            .get_access_token()
            .await
            .map_err(FcmClientError::Oauth)?;

        // https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.oauth_client.get_project_id()
        );

        let request = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&MessageWrapper::new(message.as_ref(), self.dry_run))
            .build()?;

        let response = self.http_client.execute(request).await?;
        let retry_after = response.headers().get(RETRY_AFTER);
        let retry_after = if let Some(header_value) = retry_after {
            let header_str = header_value
                .to_str()
                .map_err(|_| FcmClientError::RetryAfterHttpHeaderIsNotString)?;
            let value =
                header_str
                    .parse::<RetryAfter>()
                    .map_err(|error| FcmClientError::RetryAfterHttpHeaderInvalid {
                        error,
                        value: header_str.to_string(),
                    })?;
            Some(value)
        } else {
            None
        };
        let http_status_code = response.status().as_u16();
        // Return if I/O error occurs
        let response_body = response.bytes().await?;
        let response_json_object = serde_json::from_slice::<serde_json::Map<String, serde_json::Value>>(&response_body)
            .ok()
            .unwrap_or_default();

        Ok(FcmResponse::new(http_status_code, response_json_object, retry_after))
    }
}
