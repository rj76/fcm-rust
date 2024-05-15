pub use chrono::{DateTime, Duration, FixedOffset};
use gauth::serv_account::errors::{GetAccessTokenError, ServiceAccountBuildError};
use serde::Deserialize;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

/// A description of what went wrong with the push notification.
/// Referred from [Firebase documentation](https://firebase.google.com/docs/cloud-messaging/http-server-ref#table9)
#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ErrorReason {
    /// Check that the request contains a registration token (in the `to` or
    /// `registration_ids` field).
    MissingRegistration,

    /// Check the format of the registration token you pass to the server. Make
    /// sure it matches the registration token the client app receives from
    /// registering with Firebase Notifications. Do not truncate or add
    /// additional characters.
    InvalidRegistration,

    /// An existing registration token may cease to be valid in a number of
    /// scenarios, including:
    ///
    /// * If the client app unregisters with FCM.
    /// * If the client app is automatically unregistered, which can happen if
    ///   the user uninstalls the application. For example, on iOS, if the APNS
    ///   Feedback Service reported the APNS token as invalid.
    /// * If the registration token expires (for example, Google might decide to
    ///   refresh registration tokens, or the APNS token has expired for iOS
    ///   devices).
    /// * If the client app is updated but the new version is not configured to
    ///   receive messages.
    ///
    /// For all these cases, remove this registration token from the app server
    /// and stop using it to send messages.
    NotRegistered,

    /// Make sure the message was addressed to a registration token whose
    /// package name matches the value passed in the request.
    InvalidPackageName,

    /// A registration token is tied to a certain group of senders. When a
    /// client app registers for FCM, it must specify which senders are allowed
    /// to send messages. You should use one of those sender IDs when sending
    /// messages to the client app. If you switch to a different sender, the
    /// existing registration tokens won't work.
    MismatchSenderId,

    /// Check that the provided parameters have the right name and type.
    InvalidParameters,

    /// Check that the total size of the payload data included in a message does
    /// not exceed FCM limits: 4096 bytes for most messages, or 2048 bytes in
    /// the case of messages to topics. This includes both the keys and the
    /// values.
    MessageTooBig,

    /// Check that the custom payload data does not contain a key (such as
    /// `from`, or `gcm`, or any value prefixed by google) that is used
    /// internally by FCM. Note that some words (such as `collapse_key`) are
    /// also used by FCM but are allowed in the payload, in which case the
    /// payload value will be overridden by the FCM value.
    InvalidDataKey,

    /// Check that the value used in `time_to_live` is an integer representing a
    /// duration in seconds between 0 and 2,419,200 (4 weeks).
    InvalidTtl,

    /// In internal use only. Check
    /// [FcmError::ServerError](enum.FcmError.html#variant.ServerError).
    Unavailable,

    /// In internal use only. Check
    /// [FcmError::ServerError](enum.FcmError.html#variant.ServerError).
    InternalServerError,

    /// The rate of messages to a particular device is too high. If an iOS app
    /// sends messages at a rate exceeding APNs limits, it may receive this
    /// error message
    ///
    /// Reduce the number of messages sent to this device and use exponential
    /// backoff to retry sending.
    DeviceMessageRateExceeded,

    /// The rate of messages to subscribers to a particular topic is too high.
    /// Reduce the number of messages sent for this topic and use exponential
    /// backoff to retry sending.
    TopicsMessageRateExceeded,

    /// A message targeted to an iOS device could not be sent because the
    /// required APNs authentication key was not uploaded or has expired. Check
    /// the validity of your development and production credentials.
    InvalidApnsCredential,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub message_id: Option<u64>,
    pub error: Option<ErrorReason>,
    pub multicast_id: Option<i64>,
    pub success: Option<u64>,
    pub failure: Option<u64>,
    pub canonical_ids: Option<u64>,
    pub results: Option<Vec<MessageResult>>,
}

#[derive(Deserialize, Debug)]
pub struct MessageResult {
    pub message_id: Option<String>,
    pub registration_id: Option<String>,
    pub error: Option<ErrorReason>,
}

/// Fatal errors. Referred from [Firebase
/// documentation](https://firebase.google.com/docs/cloud-messaging/http-server-ref#table9)
#[derive(Error, Debug)]
pub enum SendError {
    #[error("Error getting access token: {0}")]
    AccessToken(GetAccessTokenError),

    #[error("Error sending message: {0}")]
    HttpRequest(reqwest::Error),

    #[error("Unknown response")]
    UnknownHttpResponse {
        status: reqwest::StatusCode,
        body: reqwest::Result<String>,
    }

    // TODO retry after error

    // TODO error variant for invalid authentication
}

#[cfg(feature = "dotenv")]
#[derive(Error, Debug)]
pub enum DotEnvClientBuildError {
    #[error("Error getting dotenv variable: {0}")]
    DotEnv(dotenv::Error),

    #[error("Error reading file from dotenv variable: {0}")]
    ReadFile(std::io::Error),

    #[error("Error parsing file from dotenv variable: {0}")]
    ParseFile(serde_json::Error),

    #[error("Error building client: {0}")]
    ClientBuild(ClientBuildError),
}

#[derive(Error, Debug)]
pub enum ClientBuildError {
    #[error("Error initializing service account: {0}")]
    ServiceAccountBuild(ServiceAccountBuildError),

    #[error("Error getting initial access token: {0}")]
    GetAccessToken(GetAccessTokenError),
}

#[derive(PartialEq, Debug)]
pub enum RetryAfter {
    /// Amount of time to wait until retrying the message is allowed.
    Delay(Duration),

    /// A point in time until retrying the message is allowed.
    DateTime(DateTime<FixedOffset>),
}

#[derive(Error, Debug)]
pub enum RetryAfterParseError {
    #[error("Error parsing Retry-After header as int: {0}")]
    IntParse(ParseIntError),

    #[error("Error parsing Retry-After header as DateTime: {0}")]
    DateParse(chrono::format::ParseError),
}

impl FromStr for RetryAfter {
    type Err = RetryAfterParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i64>()
            .map_err(RetryAfterParseError::IntParse)
            .map(Duration::seconds)
            .map(RetryAfter::Delay)
            .or_else(|_| DateTime::parse_from_rfc2822(s).map(RetryAfter::DateTime))
            .map_err(RetryAfterParseError::DateParse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Duration};
    use serde_json::json;

    #[test]
    fn test_some_errors() {
        let errors = vec![
            ("MissingRegistration", ErrorReason::MissingRegistration),
            ("InvalidRegistration", ErrorReason::InvalidRegistration),
            ("NotRegistered", ErrorReason::NotRegistered),
            ("InvalidPackageName", ErrorReason::InvalidPackageName),
            ("MismatchSenderId", ErrorReason::MismatchSenderId),
            ("InvalidParameters", ErrorReason::InvalidParameters),
            ("MessageTooBig", ErrorReason::MessageTooBig),
            ("InvalidDataKey", ErrorReason::InvalidDataKey),
            ("InvalidTtl", ErrorReason::InvalidTtl),
            ("Unavailable", ErrorReason::Unavailable),
            ("InternalServerError", ErrorReason::InternalServerError),
            ("DeviceMessageRateExceeded", ErrorReason::DeviceMessageRateExceeded),
            ("TopicsMessageRateExceeded", ErrorReason::TopicsMessageRateExceeded),
            ("InvalidApnsCredential", ErrorReason::InvalidApnsCredential),
        ];

        for (error_str, error_enum) in errors.into_iter() {
            let response_data = json!({
                "error": error_str,
                "results": [
                    {"error": error_str}
                ]
            });

            let response_string = serde_json::to_string(&response_data).unwrap();
            let fcm_response = serde_json::from_str::<Response>(&response_string).unwrap();

            assert_eq!(Some(error_enum), fcm_response.results.unwrap()[0].error);

            assert_eq!(Some(error_enum), fcm_response.error)
        }
    }

    #[test]
    fn test_retry_after_from_seconds() {
        assert_eq!(RetryAfter::Delay(Duration::seconds(420)), "420".parse().unwrap());
    }

    #[test]
    fn test_retry_after_from_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date).unwrap();

        assert_eq!(
            RetryAfter::DateTime(DateTime::parse_from_rfc2822(date).unwrap()),
            retry_after,
        );
    }
}
