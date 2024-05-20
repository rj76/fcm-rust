pub mod fcm_options;
pub mod target;

#[cfg(test)]
mod tests;

use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;

use crate::android::android_config::AndroidConfig;
use crate::android::android_config::AndroidConfigInternal;
use crate::apns::apns_config::ApnsConfig;
use crate::apns::apns_config::ApnsConfigInternal;
use crate::notification::Notification;
use crate::notification::NotificationInternal;
use crate::web::webpush_config::WebpushConfig;
use crate::web::webpush_config::WebpushConfigInternal;

use self::fcm_options::FcmOptions;
use self::fcm_options::FcmOptionsInternal;
use self::target::Target;

fn output_target<S>(target: &Target, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = s.serialize_map(Some(1))?;
    match target {
        Target::Token(token) => map.serialize_entry("token", token.as_str())?,
        Target::Topic(topic) => map.serialize_entry("topic", topic.as_str())?,
        Target::Condition(condition) => map.serialize_entry("condition", condition.as_str())?,
    }
    map.end()
}

#[derive(Serialize, Debug)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#resource:-message
pub(crate) struct MessageInternal {
    /// Arbitrary key/value payload, which must be UTF-8 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    /// Basic notification template to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<NotificationInternal>,

    /// Android specific options for messages sent through FCM connection server.
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfigInternal>,

    /// Webpush protocol options.
    #[serde(skip_serializing_if = "Option::is_none")]
    webpush: Option<WebpushConfigInternal>,

    /// Apple Push Notification Service specific options.
    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfigInternal>,

    /// Template for FCM SDK feature options to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<FcmOptionsInternal>,

    /// Target to send a message to.
    #[serde(flatten, serialize_with = "output_target")]
    target: Target,
}

/// A `Message` instance is the main object to send to the FCM API.
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#resource:-message
#[derive(Debug)]
pub struct Message {
    /// Arbitrary key/value payload, which must be UTF-8 encoded. Values must be strings.
    pub data: Option<Value>,
    /// Basic notification template to use across all platforms.
    pub notification: Option<Notification>,
    /// Target to send a message to.
    pub target: Target,
    /// Android specific options for messages sent through FCM connection server.
    pub android: Option<AndroidConfig>,
    /// Webpush protocol options.
    pub webpush: Option<WebpushConfig>,
    /// Apple Push Notification Service specific options.
    pub apns: Option<ApnsConfig>,
    /// Template for FCM SDK feature options to use across all platforms.
    pub fcm_options: Option<FcmOptions>,
}

impl Message {
    /// Complete the build and get a `MessageInternal` instance
    pub(crate) fn finalize(self) -> MessageInternal {
        MessageInternal {
            data: self.data,
            notification: self.notification.map(|n| n.finalize()),
            android: self.android.map(|a| a.finalize()),
            webpush: self.webpush.map(|w| w.finalize()),
            apns: self.apns.map(|a| a.finalize()),
            fcm_options: self.fcm_options.map(|f| f.finalize()),
            target: self.target,
        }
    }
}

/// Wrap the message in a "message" field
#[derive(Serialize)]
pub(crate) struct MessageWrapper {
    #[serde(rename = "message")]
    message: MessageInternal,
}

impl MessageWrapper {
    pub fn new(message: MessageInternal) -> MessageWrapper {
        MessageWrapper { message }
    }
}
