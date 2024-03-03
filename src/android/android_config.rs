use serde::Serialize;
use serde_json::Value;

use super::{
    android_fcm_options::{AndroidFcmOptions, AndroidFcmOptionsInternal},
    android_message_priority::AndroidMessagePriority,
    android_notification::{AndroidNotification, AndroidNotificationInternal},
};

#[derive(Serialize, Debug)]
pub(crate) struct AndroidConfigInternal {
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<AndroidMessagePriority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<AndroidNotificationInternal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<AndroidFcmOptionsInternal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    direct_boot_ok: Option<bool>,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig
pub struct AndroidConfig {
    /// An identifier of a group of messages that can be collapsed, so that only the last message gets
    /// sent when delivery can be resumed.
    pub collapse_key: Option<String>,

    /// Message priority.
    pub priority: Option<AndroidMessagePriority>,

    /// How long (in seconds) the message should be kept in FCM storage if the device is offline.
    /// Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub ttl: Option<String>,

    /// Package name of the application where the registration token must match in order to receive the message.
    pub restricted_package_name: Option<String>,

    /// Arbitrary key/value payload.
    pub data: Option<Value>,

    /// Notification to send to android devices.
    pub notification: Option<AndroidNotification>,

    /// Options for features provided by the FCM SDK for Android.
    pub fcm_options: Option<AndroidFcmOptions>,

    /// If set to true, messages will be allowed to be delivered to the app while the device is in direct boot mode.
    pub direct_boot_ok: Option<bool>,
}

impl AndroidConfig {
    pub(crate) fn finalize(self) -> AndroidConfigInternal {
        AndroidConfigInternal {
            collapse_key: self.collapse_key,
            priority: self.priority,
            ttl: self.ttl,
            restricted_package_name: self.restricted_package_name,
            data: self.data,
            notification: self.notification.map(|n| n.finalize()),
            fcm_options: self.fcm_options.map(|f| f.finalize()),
            direct_boot_ok: self.direct_boot_ok,
        }
    }
}
