use std::path::PathBuf;

use yup_oauth2::authenticator::{Authenticator, DefaultHyperClient, HyperClientBuilder};
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::ServiceAccountAuthenticator;

const FIREBASE_OAUTH_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

#[derive(thiserror::Error, Debug)]
pub enum YupOauth2Error {
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

impl YupOauth2Error {
    /// If this is `true` then most likely current service account
    /// key is invalid.
    pub(crate) fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool {
        matches!(
            self,
            YupOauth2Error::AccessTokenIsMissing |
            YupOauth2Error::Oauth(
                yup_oauth2::Error::MissingAccessToken |
                yup_oauth2::Error::AuthError(_)
            )
        )
    }
}

pub(crate) struct YupOauth2 {
    authenticator: Authenticator<HttpsConnector<HttpConnector>>,
    project_id: String,
}

impl YupOauth2 {
    pub async fn create_with_key_file(
        service_account_key_path: PathBuf,
        token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, YupOauth2Error> {
        let file = tokio::fs::read_to_string(&service_account_key_path).await
            .map_err(YupOauth2Error::ServiceAccountKeyReadingFailed)?;
        Self::create_with_string_key(file, token_cache_json_path).await
    }

    pub async fn create_with_string_key(
        service_account_key_json_string: String,
        token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, YupOauth2Error> {
        let key = yup_oauth2::parse_service_account_key(service_account_key_json_string)
            .map_err(YupOauth2Error::ServiceAccountKeyReadingFailed)?;
        let oauth_client = DefaultHyperClient.build_hyper_client()
            .map_err(YupOauth2Error::Oauth)?;
        let builder = ServiceAccountAuthenticator::with_client(key.clone(), oauth_client);
        let builder = if let Some(path) = token_cache_json_path {
            builder.persist_tokens_to_disk(path)
        } else {
            builder
        };
        let authenticator = builder.build()
            .await
            .map_err(YupOauth2Error::AuthenticatorCreatingFailed)?;

        let project_id = key.project_id
            .ok_or(YupOauth2Error::ProjectIdIsMissing)?;

        Ok(YupOauth2 {
            authenticator,
            project_id,
        })
    }

    pub async fn get_access_token(&self) -> Result<String, YupOauth2Error> {
        let scopes = [FIREBASE_OAUTH_SCOPE];
        let access_token = self.authenticator.token(&scopes).await?;
        let access_token = access_token.token()
            .ok_or(YupOauth2Error::AccessTokenIsMissing)?;

        Ok(access_token.to_string())
    }

    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }
}
