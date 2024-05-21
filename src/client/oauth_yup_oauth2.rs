use std::path::PathBuf;

use yup_oauth2::authenticator::{Authenticator, DefaultHyperClient, HyperClientBuilder};
use yup_oauth2::hyper::client::HttpConnector;
use yup_oauth2::hyper_rustls::HttpsConnector;
use yup_oauth2::ServiceAccountAuthenticator;

use super::{OauthClient, OauthErrorInfo, FIREBASE_OAUTH_SCOPE};

#[derive(thiserror::Error, Debug)]
pub enum FcmOauthError {
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

impl OauthErrorInfo for FcmOauthError {
    fn is_access_token_missing_even_if_server_requests_completed(&self) -> bool {
        matches!(
            self,
            FcmOauthError::AccessTokenIsMissing |
            FcmOauthError::Oauth(yup_oauth2::Error::AuthError(_))
        )
    }
}

pub struct OauthClientImpl {
    authenticator: Authenticator<HttpsConnector<HttpConnector>>,
    project_id: String,
}

impl OauthClient for OauthClientImpl {
    type Error = FcmOauthError;

    async fn create_with_key_file(
        service_account_key_path: PathBuf,
        token_cache_json_path: Option<PathBuf>,
    ) -> Result<Self, FcmOauthError> {
        let key = yup_oauth2::read_service_account_key(service_account_key_path)
            .await
            .map_err(FcmOauthError::ServiceAccountKeyReadingFailed)?;
        let oauth_client = DefaultHyperClient.build_hyper_client()
            .map_err(FcmOauthError::Oauth)?;
        let builder = ServiceAccountAuthenticator::with_client(key.clone(), oauth_client);
        let builder = if let Some(path) = token_cache_json_path {
            builder.persist_tokens_to_disk(path)
        } else {
            builder
        };
        let authenticator = builder.build()
            .await
            .map_err(FcmOauthError::AuthenticatorCreatingFailed)?;

        let project_id = key.project_id
            .ok_or(FcmOauthError::ProjectIdIsMissing)?;

        Ok(OauthClientImpl {
            authenticator,
            project_id,
        })
    }

    async fn get_access_token(&self) -> Result<String, FcmOauthError> {
        let scopes = [FIREBASE_OAUTH_SCOPE];
        let access_token = self.authenticator.token(&scopes).await?;
        let access_token = access_token.token()
            .ok_or(FcmOauthError::AccessTokenIsMissing)?;

        Ok(access_token.to_string())
    }

    fn get_project_id(&self) -> &str {
        &self.project_id
    }
}
