pub(crate) mod response;

use crate::client::response::{Error, ErrorReason, Response, RetryAfter};
use crate::{Message, MessageInternal};
pub use gauth;
use gauth::serv_account::{ServiceAccount, ServiceAccountBuilder, ServiceAccountKey};
use reqwest::header::RETRY_AFTER;
use reqwest::{Client as HttpClient, StatusCode};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
/// An async client for sending the notification payload.
pub struct Client {
    http_client: HttpClient,
    service_account: Arc<ServiceAccount>,
    project_id: Arc<String>,
}

// will be used to wrap the message in a "message" field
#[derive(Serialize)]
struct MessageWrapper<'a> {
    #[serde(rename = "message")]
    message: &'a MessageInternal,
}

impl MessageWrapper<'_> {
    fn new(message: &MessageInternal) -> MessageWrapper {
        MessageWrapper { message }
    }
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get a new instance of Client.
    #[cfg(feature = "dotenv")]
    pub fn new() -> Result<Client, Error> {
        let path = dotenv::var("GOOGLE_APPLICATION_CREDENTIALS").map_err(Error::DotEnv)?;
        let bytes = std::fs::read(path).map_err(Error::ReadFile)?;
        let key = serde_json::from_slice::<ServiceAccountKey>(&bytes).map_err(Error::ParseFile)?;
        Ok(Self::from_key(key))
    }

    pub fn from_key(key: ServiceAccountKey) -> Client {
        Self::builder().build(key)
    }

    pub async fn send(&self, message: Message) -> Result<Response, Error> {
        let fin = message.finalize();
        let wrapper = MessageWrapper::new(&fin);

        let access_token = self.service_account.access_token().await.map_err(Error::AccessToken)?;

        // https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let request = self
            .http_client
            .post(&url)
            .bearer_auth(access_token.bearer_token)
            .json(&wrapper)
            .build()?;

        let response = self.http_client.execute(request).await?;

        let response_status = response.status();

        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|ra| ra.to_str().ok())
            .and_then(|ra| ra.parse::<RetryAfter>().ok());

        match response_status {
            StatusCode::OK => {
                let fcm_response = response.json::<Response>().await.unwrap();

                match fcm_response.error {
                    Some(ErrorReason::Unavailable) => Err(Error::ServerError(retry_after)),
                    Some(ErrorReason::InternalServerError) => Err(Error::ServerError(retry_after)),
                    _ => Ok(fcm_response),
                }
            }
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::BAD_REQUEST => {
                let body = response.text().await.unwrap();
                Err(Error::InvalidMessage(format!("Bad Request ({body}")))
            }
            status if status.is_server_error() => Err(Error::ServerError(retry_after)),
            _ => Err(Error::InvalidMessage("Unknown Error".to_string())),
        }
    }
}

pub struct ClientBuilder {
    http_client: Option<HttpClient>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self { http_client: None }
    }

    pub fn http_client(mut self, http_client: HttpClient) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self, key: ServiceAccountKey) -> Client {
        let http_client = self.http_client.unwrap_or_default();
        let project_id = key.project_id.clone();
        let service_account = ServiceAccountBuilder::new()
            .key(key)
            .scopes(vec!["https://www.googleapis.com/auth/firebase.messaging"])
            .http_client(http_client.clone())
            .build();

        Client {
            http_client,
            project_id: Arc::new(project_id),
            service_account: Arc::new(service_account),
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
