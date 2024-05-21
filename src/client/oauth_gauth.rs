use std::path::{Path, PathBuf};

use gauth::serv_account::ServiceAccount;

use super::{OauthClient, FIREBASE_OAUTH_SCOPE};

#[derive(thiserror::Error, Debug)]
pub enum FcmOauthError {
    #[error("OAuth error: {0}")]
    Oauth(String),
    #[error("Service account key path is not UTF-8")]
    ServiceAccountKeyPathIsNotUtf8,
    #[error("Service account key reading failed: {0}")]
    ServiceAccountKeyReadingFailed(std::io::Error),
    #[error("Service account key JSON deserialization failed: {0}")]
    ServiceAccountKeyDeserializationFailed(serde_json::Error),
    #[error("Service account key JSON does not contain project ID")]
    ProjectIdIsMissing,
}

pub struct OauthClientImpl {
    project_id: String,
    service_account_key_path: String,
}

impl OauthClient for OauthClientImpl {
    type Error = FcmOauthError;

    async fn create_with_key_file(
        service_account_key_path: PathBuf,
        _token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, FcmOauthError> {
        Ok(OauthClientImpl {
            project_id: get_project_id(&service_account_key_path)?,
            service_account_key_path: service_account_key_path.to_str()
                .ok_or(FcmOauthError::ServiceAccountKeyPathIsNotUtf8)?
                .to_string(),
        })
    }

    async fn get_access_token(&self) -> Result<String, FcmOauthError> {
        let scopes = vec![FIREBASE_OAUTH_SCOPE];
        let mut service_account = ServiceAccount::from_file(&self.service_account_key_path, scopes);
        let access_token = service_account.access_token().await
            .map_err(|e| e.to_string())
            .map_err(FcmOauthError::Oauth)?;

        let token_no_bearer = access_token.split(char::is_whitespace).collect::<Vec<&str>>()[1];

        Ok(token_no_bearer.to_string())
    }

    fn get_project_id(&self) -> &str {
        &self.project_id
    }
}

fn read_service_key_file_json(service_account_key_path: impl AsRef<Path>) -> Result<serde_json::Value, FcmOauthError> {
    let json_string = std::fs::read_to_string(service_account_key_path)
        .map_err(FcmOauthError::ServiceAccountKeyReadingFailed)?;
    let json_content: serde_json::Value = serde_json::from_str(&json_string)
        .map_err(FcmOauthError::ServiceAccountKeyDeserializationFailed)?;

    Ok(json_content)
}

fn get_project_id(service_account_key_path: impl AsRef<Path>) -> Result<String, FcmOauthError> {
    let json_content = read_service_key_file_json(service_account_key_path)?;
    let project_id = json_content["project_id"].as_str()
        .ok_or(FcmOauthError::ProjectIdIsMissing)?;
    Ok(project_id.to_string())
}
