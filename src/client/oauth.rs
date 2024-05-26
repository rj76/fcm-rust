use std::path::PathBuf;

use yup_oauth2::authenticator::{Authenticator, DefaultHyperClient, HyperClientBuilder};
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::ServiceAccountAuthenticator;

const FIREBASE_OAUTH_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

#[derive(thiserror::Error, Debug)]
pub enum OauthError {
    #[error("Service account key reading failed: {0}")]
    ServiceAccountKeyReadingFailed(std::io::Error),
    #[error("OAuth error: {0}")]
    Oauth(#[from] yup_oauth2::Error),
    #[error("Access token is missing")]
    AccessTokenIsMissing,
    #[error("Authenticator creation failed: {0}")]
    AuthenticatorCreatingFailed(std::io::Error),
    #[error("Service account key JSON does not contain project ID")]
    ProjectIdIsMissing,
}

impl OauthError {
    /// If this is `true` then most likely current service account
    /// key is invalid.
    pub(crate) fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool {
        matches!(
            self,
            OauthError::AccessTokenIsMissing |
            OauthError::Oauth(
                yup_oauth2::Error::MissingAccessToken |
                yup_oauth2::Error::AuthError(_)
            )
        )
    }
}

pub(crate) struct OauthClient {
    authenticator: Authenticator<HttpsConnector<HttpConnector>>,
    project_id: String,
}

impl OauthClient {
    pub async fn create_with_key_file(
        service_account_key_path: PathBuf,
        token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, OauthError> {
        let file = tokio::fs::read_to_string(&service_account_key_path).await
            .map_err(OauthError::ServiceAccountKeyReadingFailed)?;
        Self::create_with_string_key(file, token_cache_json_path).await
    }

    pub async fn create_with_string_key(
        service_account_key_json_string: String,
        token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, OauthError> {
        let key = yup_oauth2::parse_service_account_key(service_account_key_json_string)
            .map_err(OauthError::ServiceAccountKeyReadingFailed)?;
        let oauth_client = DefaultHyperClient.build_hyper_client()
            .map_err(OauthError::Oauth)?;
        let builder = ServiceAccountAuthenticator::with_client(key.clone(), oauth_client);
        let builder = if let Some(path) = token_cache_json_path {
            builder.persist_tokens_to_disk(path)
        } else {
            builder
        };
        let authenticator = builder.build()
            .await
            .map_err(OauthError::AuthenticatorCreatingFailed)?;

        let project_id = key.project_id
            .ok_or(OauthError::ProjectIdIsMissing)?;

        Ok(OauthClient {
            authenticator,
            project_id,
        })
    }

    pub async fn get_access_token(&self) -> Result<String, OauthError> {
        let scopes = [FIREBASE_OAUTH_SCOPE];
        let access_token = self.authenticator.token(&scopes).await?;
        let access_token = access_token.token()
            .ok_or(OauthError::AccessTokenIsMissing)?;

        Ok(access_token.to_string())
    }

    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }
}
