use std::path::{Path, PathBuf};

use gauth::serv_account::ServiceAccount;

use super::{OauthClient, OauthError, FIREBASE_OAUTH_SCOPE};

#[derive(thiserror::Error, Debug)]
pub enum GauthError {
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

impl OauthError for GauthError {}

pub struct Gauth {
    project_id: String,
    service_account_key_path: String,
}

impl OauthClient for Gauth {
    type Error = GauthError;

    async fn create_with_key_file(
        service_account_key_path: PathBuf,
        _token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, GauthError> {
        Ok(Gauth {
            project_id: get_project_id(&service_account_key_path)?,
            service_account_key_path: service_account_key_path.to_str()
                .ok_or(GauthError::ServiceAccountKeyPathIsNotUtf8)?
                .to_string(),
        })
    }

    async fn get_access_token(&self) -> Result<String, GauthError> {
        let scopes = vec![FIREBASE_OAUTH_SCOPE];
        let mut service_account = ServiceAccount::from_file(&self.service_account_key_path, scopes);
        let access_token = service_account.access_token().await
            .map_err(|e| e.to_string())
            .map_err(GauthError::Oauth)?;

        let token_no_bearer = access_token.split(char::is_whitespace).collect::<Vec<&str>>()[1];

        Ok(token_no_bearer.to_string())
    }

    fn get_project_id(&self) -> &str {
        &self.project_id
    }
}

fn read_service_key_file_json(service_account_key_path: impl AsRef<Path>) -> Result<serde_json::Value, GauthError> {
    let json_string = std::fs::read_to_string(service_account_key_path)
        .map_err(GauthError::ServiceAccountKeyReadingFailed)?;
    let json_content: serde_json::Value = serde_json::from_str(&json_string)
        .map_err(GauthError::ServiceAccountKeyDeserializationFailed)?;

    Ok(json_content)
}

fn get_project_id(service_account_key_path: impl AsRef<Path>) -> Result<String, GauthError> {
    let json_content = read_service_key_file_json(service_account_key_path)?;
    let project_id = json_content["project_id"].as_str()
        .ok_or(GauthError::ProjectIdIsMissing)?;
    Ok(project_id.to_string())
}
