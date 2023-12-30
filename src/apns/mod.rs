use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsconfig
pub struct ApnsConfig {
    // HTTP request headers defined in Apple Push Notification Service.
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Value>,

    // APNs payload as a JSON object, including both aps dictionary and custom payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<Value>,

    // Options for features provided by the FCM SDK for iOS.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<ApnsFcmOptions>,
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsfcmoptions
pub struct ApnsFcmOptions {
    // Label associated with the message's analytics data.
    analytics_label: String,

    // Contains the URL of an image that is going to be displayed in a notification.
    image: String
}
