use serde::Serialize;
use serde_json::Value;

use super::apns_fcm_options::ApnsFcmOptions;

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsconfig>
pub struct ApnsConfig {
    /// HTTP request headers defined in Apple Push Notification Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,

    /// APNs payload as a JSON object, including both aps dictionary and custom payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,

    /// Options for features provided by the FCM SDK for iOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<ApnsFcmOptions>,
}
