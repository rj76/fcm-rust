use reqwest::header::RETRY_AFTER;

use crate::{message::{Message, MessageWrapper}, FcmClientBuilder, FcmClientError, FcmResponse, OauthClient, RetryAfter};

use super::OauthClientInternal;

pub(crate) struct FcmClientInternal<T: OauthClient> {
    http_client: reqwest::Client,
    oauth_client: T,
}

impl <T: OauthClientInternal> FcmClientInternal<T> {
    pub async fn new_from_builder(
        fcm_builder: FcmClientBuilder<T>,
    ) -> Result<Self, FcmClientError<T::Error>> {
        let builder = reqwest::ClientBuilder::new();
        let builder = if let Some(timeout) = fcm_builder.fcm_request_timeout {
            builder.timeout(timeout)
        } else {
            builder
        };
        let http_client = builder.build()?;

        let oauth_client = if let Some(key_json) = fcm_builder.service_account_key_json_string {
            T::create_with_string_key(
                key_json,
                fcm_builder.token_cache_json_path,
            )
                .await
                .map_err(FcmClientError::Oauth)?
        } else {
            let service_account_key_path = if let Some(path) = fcm_builder.service_account_key_json_path {
                path
            } else {
                dotenvy::var("GOOGLE_APPLICATION_CREDENTIALS")?.into()
            };

            T::create_with_key_file(
                service_account_key_path,
                fcm_builder.token_cache_json_path,
            )
                .await
                .map_err(FcmClientError::Oauth)?
        };

        Ok(FcmClientInternal {
            http_client,
            oauth_client,
        })
    }
}

impl <T: OauthClientInternal> FcmClientInternal<T> {
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

        Ok(FcmResponse::new(
            http_status_code,
            response_json_object,
            retry_after,
        ))
    }
}
