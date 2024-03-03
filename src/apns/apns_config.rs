use serde::Serialize;
use serde_json::Value;

use super::apns_fcm_options::{ApnsFcmOptions, ApnsFcmOptionsInternal};

#[derive(Serialize, Debug)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsconfig
pub(crate) struct ApnsConfigInternal {
    /// HTTP request headers defined in Apple Push Notification Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    /// APNs payload as a JSON object, including both aps dictionary and custom payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,

    /// Options for features provided by the FCM SDK for iOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptionsInternal>,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsconfig
pub struct ApnsConfig {
    /// HTTP request headers defined in Apple Push Notification Service.
    pub headers: Option<Value>,
    /// APNs payload as a JSON object, including both aps dictionary and custom payload.
    pub payload: Option<Value>,
    /// Options for features provided by the FCM SDK for iOS.
    pub fcm_options: Option<ApnsFcmOptions>,
}

impl ApnsConfig {
    pub(crate) fn finalize(self) -> ApnsConfigInternal {
        ApnsConfigInternal {
            headers: self.headers,
            payload: self.payload,
            fcm_options: self.fcm_options.map(|fcm_options| fcm_options.finalize()),
        }
    }
}
