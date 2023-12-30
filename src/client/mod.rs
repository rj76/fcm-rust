pub(crate) mod response;

use gauth::serv_account::ServiceAccount;
use reqwest::{Body, StatusCode};
use reqwest::header::RETRY_AFTER;
use crate::client::response::{ErrorReason, FcmError, FcmResponse, RetryAfter};
use crate::Message;


/// An async client for sending the notification payload.
pub struct Client {
    http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Get a new instance of Client.
    pub fn new() -> Client {
        let http_client = reqwest::ClientBuilder::new()
            .pool_max_idle_per_host(usize::MAX)
            .build()
            .unwrap();

        Client { http_client }
    }

    fn get_service_key_file_name(&self) -> Result<String, String> {
        let key_path = match dotenv::var("GOOGLE_APPLICATION_CREDENTIALS") {
            Ok(key_path) => key_path,
            Err(err) => return Err(err.to_string()),
        };

        Ok(key_path)
    }

    fn read_service_key_file(&self) -> Result<String, String> {
        let key_path = self.get_service_key_file_name()?;

        let private_key_content = match std::fs::read(key_path) {
            Ok(content) => content,
            Err(err) => return Err(err.to_string()),
        };

        Ok(String::from_utf8(private_key_content).unwrap())
    }

    fn read_service_key_file_json(&self) -> Result<serde_json::Value, String> {
        let file_content = match self.read_service_key_file() {
            Ok(content) => content,
            Err(err) => return Err(err),
        };

        let json_content: serde_json::Value = match serde_json::from_str(&file_content) {
            Ok(json) => json,
            Err(err) => return Err(err.to_string()),
        };

        Ok(json_content)
    }

    fn get_project_id(&self) -> Result<String, String> {
        let json_content = match self.read_service_key_file_json() {
            Ok(json) => json,
            Err(err) => return Err(err),
        };

        let project_id = match json_content["project_id"].as_str() {
            Some(project_id) => project_id,
            None => return Err("could not get project_id".to_string()),
        };

        Ok(project_id.to_string())
    }

    async fn get_auth_token(&self) -> Result<String, String> {
        let tkn = match self.access_token().await {
            Ok(tkn) => tkn,
            Err(_) => return Err("could not get access token".to_string()),
        };

        Ok(tkn)
    }

    pub async fn access_token(&self) -> Result<String, String> {
        let scopes = vec!["https://www.googleapis.com/auth/firebase.messaging"];
        let key_path = self.get_service_key_file_name()?;

        let mut service_account = ServiceAccount::from_file(&key_path, scopes);
        let access_token = match service_account.access_token().await {
            Ok(access_token) => access_token,
            Err(err) => return Err(err.to_string()),
        };

        let token_no_bearer = access_token.split(" ").collect::<Vec<&str>>()[1];

        Ok(token_no_bearer.to_string())
    }

    pub async fn send(&self, message: Message) -> Result<FcmResponse, FcmError> {
        let payload = serde_json::to_vec(&message).unwrap();

        let project_id = match self.get_project_id() {
            Ok(project_id) => project_id,
            Err(err) => return Err(FcmError::ProjectIdError(err)),
        };

        let auth_token = match self.get_auth_token().await {
            Ok(tkn) => tkn,
            Err(err) => return Err(FcmError::ProjectIdError(err)),
        };

        // https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages/send
        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            project_id
        );

        let request = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .bearer_auth(auth_token)
            .body(Body::from(payload))
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
                let fcm_response: FcmResponse = response.json().await.unwrap();

                match fcm_response.error {
                    Some(ErrorReason::Unavailable) => Err(FcmError::ServerError(retry_after)),
                    Some(ErrorReason::InternalServerError) => Err(FcmError::ServerError(retry_after)),
                    _ => Ok(fcm_response),
                }
            }
            StatusCode::UNAUTHORIZED => Err(FcmError::Unauthorized),
            StatusCode::BAD_REQUEST => Err(FcmError::InvalidMessage("Bad Request".to_string())),
            status if status.is_server_error() => Err(FcmError::ServerError(retry_after)),
            _ => Err(FcmError::InvalidMessage("Unknown Error".to_string())),
        }
    }
}
