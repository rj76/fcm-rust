pub(crate) mod response;

use crate::client::response::Response;
use crate::{
    ClientBuildError, Error404Response, Message, MessageInternal, SendError, ERROR_404_CODE_UNREGISTERED,
    TYPE_FCM_ERROR,
};
pub use gauth;
use gauth::serv_account::{ServiceAccount, ServiceAccountBuilder, ServiceAccountKey};
use reqwest::{Client as HttpClient, StatusCode};
use serde::Serialize;
use std::sync::Arc;

#[cfg(feature = "dotenv")]
use crate::DotEnvClientBuildError;

const FIREBASE_MESSAGING_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
#[cfg(feature = "dotenv")]
const ENV_VAR_FILE: &str = "GOOGLE_APPLICATION_CREDENTIALS";

/// An FCM v1 client that can be used to send messages to the FCM service. Can be constructed from a ServiceAccountKey using the `Client::builder()` method. The convenience methods `from_key()` and `new()` are also available.
///
/// Upon creation, the client will validate the provided ServiceAccountKey by requesting an initial access token and will return an error if invalid.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: HttpClient,
    service_account: Arc<ServiceAccount>,
    project_id: Arc<String>,
}

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

    /// Create a new Client using credentials from a file path specified in the GOOGLE_APPLICATION_CREDENTIALS environment variable.
    #[cfg(feature = "dotenv")]
    pub async fn new() -> Result<Client, DotEnvClientBuildError> {
        let path = dotenv::var(ENV_VAR_FILE).map_err(DotEnvClientBuildError::DotEnv)?;
        let bytes = std::fs::read(path).map_err(DotEnvClientBuildError::ReadFile)?;
        let key = serde_json::from_slice::<ServiceAccountKey>(&bytes).map_err(DotEnvClientBuildError::ParseFile)?;
        Self::from_key(key).await.map_err(DotEnvClientBuildError::ClientBuild)
    }

    pub async fn from_key(key: ServiceAccountKey) -> Result<Client, ClientBuildError> {
        Self::builder().build(key).await
    }

    pub async fn send(&self, message: Message) -> Result<Response, SendError> {
        let fin = message.finalize();
        let wrapper = MessageWrapper::new(&fin);

        let access_token = self
            .service_account
            .access_token()
            .await
            .map_err(SendError::AccessToken)?;

        // https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(access_token.bearer_token)
            .json(&wrapper)
            .send()
            .await
            .map_err(SendError::HttpRequest)?;

        let response_status = response.status();

        // let retry_after = response
        //     .headers()
        //     .get(RETRY_AFTER)
        //     .and_then(|ra| ra.to_str().ok())
        //     .and_then(|ra| ra.parse::<RetryAfter>().ok());

        match response_status {
            StatusCode::OK => response.json::<Response>().await.map_err(SendError::ResponseParse),
            StatusCode::NOT_FOUND => {
                let response = response
                    .json::<Error404Response>()
                    .await
                    .map_err(SendError::ResponseParse)?;
                for detail in response.error.details.iter() {
                    if detail.typ == TYPE_FCM_ERROR && detail.error_code == ERROR_404_CODE_UNREGISTERED {
                        return Err(SendError::Unregistered);
                    }
                }
                Err(SendError::UnknownError404Response(response))
            }
            // StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            // StatusCode::BAD_REQUEST => {
            //     let body = response.text().await.unwrap();
            //     Err(Error::InvalidMessage(format!("Bad Request ({body}")))
            // }
            // status if status.is_server_error() => Err(Error::ServerError(retry_after)),
            // _ => Err(Error::InvalidMessage("Unknown Error".to_string())),
            _ => Err(SendError::UnknownHttpResponse {
                status: response_status,
                body: response.text().await,
            }),
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

    pub async fn build(self, key: ServiceAccountKey) -> Result<Client, ClientBuildError> {
        let http_client = self.http_client.unwrap_or_default();
        let project_id = key.project_id.clone();
        let service_account = ServiceAccountBuilder::new()
            .key(key)
            .scopes(vec![FIREBASE_MESSAGING_SCOPE])
            .http_client(http_client.clone())
            .build()
            .map_err(ClientBuildError::ServiceAccountBuild)?;

        // Validate the key by requesting initial access token
        let _access_token = service_account
            .access_token()
            .await
            .map_err(ClientBuildError::GetAccessToken)?;

        Ok(Client {
            http_client,
            project_id: Arc::new(project_id),
            service_account: Arc::new(service_account),
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
