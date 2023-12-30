use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushconfig
pub struct WebpushConfig {
    // HTTP headers defined in webpush protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    // Arbitrary key/value payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    // Web Notification options as a JSON object.
    // Struct format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Struct
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Value>,

    // Options for features provided by the FCM SDK for Web.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<WebpushFcmOptions>,
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushfcmoptions
pub struct WebpushFcmOptions {
    // The link to open when the user clicks on the notification.
    link: String,

    // Label associated with the message's analytics data.
    analytics_label: String
}
