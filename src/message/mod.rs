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
use crate::fcm_options::FcmOptions;
use crate::fcm_options::FcmOptionsInternal;
use crate::notification::Notification;
use crate::notification::NotificationInternal;
use crate::web::webpush_config::WebpushConfig;
use crate::web::webpush_config::WebpushConfigInternal;

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
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#resource:-message
pub struct MessageInternal {
    // Arbitrary key/value payload, which must be UTF-8 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    // Basic notification template to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<NotificationInternal>,

    // Android specific options for messages sent through FCM connection server.
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfigInternal>,

    // Webpush protocol options.
    #[serde(skip_serializing_if = "Option::is_none")]
    webpush: Option<WebpushConfigInternal>,

    // Apple Push Notification Service specific options.
    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfigInternal>,

    // Template for FCM SDK feature options to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<FcmOptionsInternal>,

    // Target to send a message to.
    #[serde(flatten, serialize_with = "output_target")]
    target: Target,
}

///
/// A builder to get a `Message` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::{MessageBuilder, NotificationBuilder, Target};
///
/// let mut builder = MessageBuilder::new(Target::Token("token".to_string()));
/// builder.notification(NotificationBuilder::new().finalize());
/// let message = builder.finalize();
/// ```
#[derive(Debug)]
pub struct Message {
    pub data: Option<Value>,
    pub notification: Option<Notification>,
    pub target: Target,
    pub android: Option<AndroidConfig>,
    pub webpush: Option<WebpushConfig>,
    pub apns: Option<ApnsConfig>,
    pub fcm_options: Option<FcmOptions>,
}

impl Message {
    /// Complete the build and get a `Message` instance
    pub fn finalize(self) -> MessageInternal {
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
