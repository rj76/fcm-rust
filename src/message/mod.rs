pub mod fcm_options;
pub mod target;

#[cfg(test)]
mod tests;

use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;

use crate::android::android_config::AndroidConfig;
use crate::apns::apns_config::ApnsConfig;
use crate::notification::Notification;
use crate::web::webpush_config::WebpushConfig;

use self::fcm_options::FcmOptions;
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

#[derive(Debug, Serialize)]
/// A `Message` instance is the main object to send to the FCM API.
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#resource:-message>
pub struct Message {
    /// Arbitrary key/value payload, which must be UTF-8 encoded. Values must be strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,

    /// Basic notification template to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<Notification>,

    /// Android specific options for messages sent through FCM connection server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<AndroidConfig>,

    /// Webpush protocol options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webpush: Option<WebpushConfig>,

    /// Apple Push Notification Service specific options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns: Option<ApnsConfig>,

    /// Template for FCM SDK feature options to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcm_options: Option<FcmOptions>,

    /// Target to send a message to.
    #[serde(flatten, serialize_with = "output_target")]
    pub target: Target,
}

/// Wrap the message in a "message" field
#[derive(Serialize)]
pub(crate) struct MessageWrapper {
    message: Message,
}

impl MessageWrapper {
    pub fn new(message: Message) -> MessageWrapper {
        MessageWrapper { message }
    }
}
