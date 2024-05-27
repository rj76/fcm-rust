use serde::Serialize;
use serde_json::Value;

use super::webpush_fcm_options::WebpushFcmOptions;

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushconfig>
pub struct WebpushConfig {
    /// HTTP headers defined in webpush protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,

    /// Arbitrary key/value payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,

    /// Web Notification options as a JSON object.
    /// Struct format: <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Struct>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Value>,

    /// Options for features provided by the FCM SDK for Web.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<WebpushFcmOptions>,
}
