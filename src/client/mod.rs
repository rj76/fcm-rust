pub(crate) mod response;

#[cfg(feature = "gauth")]
pub mod oauth_gauth;

#[cfg(feature = "yup-oauth2")]
pub mod oauth_yup_oauth2;

use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::header::RETRY_AFTER;

use crate::client::response::{FcmResponse, RetryAfter, FcmHttpResponseStatus};
use crate::message::{Message, MessageWrapper};

#[cfg(feature = "gauth")]
pub type DefaultOauthClient = oauth_gauth::Gauth;

#[cfg(all(feature = "yup-oauth2", not(feature = "gauth")))]
pub type DefaultOauthClient = oauth_yup_oauth2::YupOauth2;

const FIREBASE_OAUTH_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

#[derive(thiserror::Error, Debug)]
pub enum FcmClientError<T: OauthError = <DefaultOauthClient as OauthClient>::Error> {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("OAuth error: {0}")]
    Oauth(T),
    #[error("Dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("Retry-After HTTP header value is not valid string")]
    RetryAfterHttpHeaderIsNotString,
    #[error("Retry-After HTTP header value is not valid, error: {error}, value: {value}")]
    RetryAfterHttpHeaderInvalid {
        error: chrono::ParseError,
        value: String,
    },
}

impl <T: OauthErrorAccessTokenStatus> FcmClientError<T> {
    /// If this is `true` then most likely current service account
    /// key is invalid.
    pub fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool {
        match self {
            FcmClientError::Oauth(error) =>
                error.is_access_token_missing_even_if_server_requests_completed(),
            _ => false,
        }
    }
}

pub trait OauthClient: Sized {
    type Error: OauthError;

    fn create_with_key_file(
        service_account_key_path: PathBuf,
        token_cache_json_path: Option<PathBuf>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send;

    fn get_access_token(
        &self
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> + Send;

    fn get_project_id(&self) -> &str;
}

pub trait OauthError: std::error::Error {}

pub trait OauthErrorAccessTokenStatus: OauthError {
    /// If this is `true` then most likely current service account
    /// key is invalid.
    fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct FcmClientBuilder<T: OauthClient> {
    service_account_key_json_path: Option<PathBuf>,
    token_cache_json_path: Option<PathBuf>,
    fcm_request_timeout: Option<Duration>,
    _phantom: std::marker::PhantomData<T>,
}

impl <T: OauthClient> FcmClientBuilder<T> {
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
    /// Google recommends at least 10 minute timeout for FCM requests.
    /// <https://firebase.google.com/docs/cloud-messaging/scale-fcm#timeouts>
    pub fn fcm_request_timeout(mut self, fcm_request_timeout: Duration) -> Self {
        self.fcm_request_timeout = Some(fcm_request_timeout);
        self
    }

    pub async fn build(self) -> Result<FcmClient<T>, FcmClientError<T::Error>> {
        FcmClient::new_from_builder(self).await
    }
}

impl <T: OauthClient> Default for FcmClientBuilder<T> {
    fn default() -> Self {
        Self {
            service_account_key_json_path: None,
            token_cache_json_path: None,
            fcm_request_timeout: None,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "yup-oauth2")]
impl FcmClientBuilder<oauth_yup_oauth2::YupOauth2> {
    /// Set path to the token cache JSON file. Default is no token cache JSON file.
    pub fn token_cache_json_path(mut self, token_cache_json_path: impl AsRef<Path>) -> Self {
        self.token_cache_json_path = Some(token_cache_json_path.as_ref().to_path_buf());
        self
    }
}

/// An async client for sending the notification payload.
pub struct FcmClient<T: OauthClient = DefaultOauthClient> {
    http_client: reqwest::Client,
    oauth_client: T,
}

impl FcmClient<DefaultOauthClient> {
    pub fn builder() -> FcmClientBuilder<DefaultOauthClient> {
        FcmClientBuilder::new()
    }
}

impl <T: OauthClient> FcmClient<T> {
    async fn new_from_builder(
        fcm_builder: FcmClientBuilder<T>,
    ) -> Result<Self, FcmClientError<T::Error>> {
        let builder = reqwest::ClientBuilder::new();
        let builder = if let Some(timeout) = fcm_builder.fcm_request_timeout {
            builder.timeout(timeout)
        } else {
            builder
        };
        let http_client = builder.build()?;

        let service_account_key_path = if let Some(path) = fcm_builder.service_account_key_json_path {
            path
        } else {
            dotenv::var("GOOGLE_APPLICATION_CREDENTIALS")?.into()
        };

        let oauth_client = T::create_with_key_file(
            service_account_key_path,
            fcm_builder.token_cache_json_path,
        )
            .await
            .map_err(FcmClientError::Oauth)?;

        Ok(FcmClient {
            http_client,
            oauth_client,
        })
    }

    pub async fn send(&self, message: Message) -> Result<FcmResponse, FcmClientError<T::Error>> {
        let access_token = self.oauth_client.get_access_token()
            .await
            .map_err(FcmClientError::Oauth)?;

        // https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send
        let url = format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", self.oauth_client.get_project_id());

        let request = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&MessageWrapper::new(message))
            .build()?;

        let response = self.http_client.execute(request).await?;
        let retry_after = response
            .headers()
            .get(RETRY_AFTER);
        let retry_after = if let Some(header_value) = retry_after {
            let header_str = header_value.to_str()
                .map_err(|_| FcmClientError::RetryAfterHttpHeaderIsNotString)?;
            let value = header_str.parse::<RetryAfter>()
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
        let response_status = FcmHttpResponseStatus::new(
            http_status_code,
            &response_json_object,
        );

        Ok(FcmResponse::new(
            response_status,
            response_json_object,
            retry_after,
        ))
    }
}
