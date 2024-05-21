use serde::Serialize;
use serde_json::Value;

use super::{
    android_fcm_options::AndroidFcmOptions,
    android_message_priority::AndroidMessagePriority,
    android_notification::AndroidNotification,
};

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig>
pub struct AndroidConfig {
    /// An identifier of a group of messages that can be collapsed, so that only the last message gets
    /// sent when delivery can be resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,

    /// Message priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<AndroidMessagePriority>,

    /// How long (in seconds) the message should be kept in FCM storage if the device is offline.
    /// Duration format: <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,

    /// Package name of the application where the registration token must match in order to receive the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,

    /// Arbitrary key/value payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,

    /// Notification to send to android devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<AndroidNotification>,

    /// Options for features provided by the FCM SDK for Android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<AndroidFcmOptions>,

    /// If set to true, messages will be allowed to be delivered to the app while the device is in direct boot mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_boot_ok: Option<bool>,
}
