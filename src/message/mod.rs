use serde::{Serialize, Serializer};
use serde_json::Value;
use crate::android::AndroidConfig;
use crate::apns::ApnsConfig;
use crate::Notification;
use crate::web::WebpushConfig;

#[cfg(test)]
mod tests;

#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Token(String),
    Topic(String),
    Condition(String)
}

fn output_target<S>(target: &Target, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    match target {
        Target::Token(token) => {
            s.serialize_newtype_struct("token", token.as_str())
            // serializer.
            // let ser = s.serialize_str(token.as_str())?;
            // s.serialize_field("token", ser)?
        },
        Target::Topic(topic) => {
            s.serialize_newtype_struct("topic", topic.as_str())
            // let ser = s.serialize_str(topic.as_str())?;
            // s.serialize_field("topic", ser)?
        },
        Target::Condition(condition) => {
            s.serialize_newtype_struct("condition", condition.as_str())
            // let ser = s.serialize_str(condition.as_str())?;
            // s.serialize_field("condition", ser)?
        },
    }
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#resource:-message
pub struct Message {
    // Arbitrary key/value payload, which must be UTF-8 encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    // Basic notification template to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<Notification>,

    // Android specific options for messages sent through FCM connection server.
    #[serde(skip_serializing_if = "Option::is_none")]
    android: Option<AndroidConfig>,

    // Webpush protocol options.
    #[serde(skip_serializing_if = "Option::is_none")]
    webpush: Option<WebpushConfig>,

    // Apple Push Notification Service specific options.
    #[serde(skip_serializing_if = "Option::is_none")]
    apns: Option<ApnsConfig>,

    // Template for FCM SDK feature options to use across all platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<FcmOptions>,

    // Target to send a message to.
    #[serde(serialize_with = "output_target")]
    target: Target
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#fcmoptions
pub struct FcmOptions {
    // Label associated with the message's analytics data.
    analytics_label: String,
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
pub struct MessageBuilder {
    data: Option<Value>,
    notification: Option<Notification>,
    target: Target
}

impl MessageBuilder {
    /// Get a new instance of Message. You need to supply to.
    pub fn new(target: Target) -> Self {
        MessageBuilder {
            data: None,
            notification: None,
            target,
        }
    }

    /// Use this to add custom key-value pairs to the message. This data
    /// must be handled appropriately on the client end. The data can be
    /// anything that Serde can serialize to JSON.
    ///
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, Target};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let mut builder = MessageBuilder::new(Target::Token("token".to_string()));
    /// builder.data(&map).expect("Should have been able to add data");
    /// let message = builder.finalize();
    /// ```
    pub fn data(&mut self, data: &dyn erased_serde::Serialize) -> Result<&mut Self, serde_json::Error> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    /// Use this to set a `Notification` for the message.
    /// # Examples:
    /// ```rust
    /// use fcm::{MessageBuilder, NotificationBuilder, Target};
    ///
    /// let mut builder = NotificationBuilder::new();
    /// builder.title("Hey!".to_string());
    /// builder.body("Do you want to catch up later?".to_string());
    /// let notification = builder.finalize();
    ///
    /// let mut builder = MessageBuilder::new(Target::Token("token".to_string()));
    /// builder.notification(notification);
    /// let message = builder.finalize();
    /// ```
    pub fn notification(&mut self, notification: Notification) -> &mut Self {
        self.notification = Some(notification);
        self
    }

    /// Complete the build and get a `Message` instance
    pub fn finalize(self) -> Message {
        Message {
            data: self.data,
            notification: self.notification,
            android: None,
            webpush: None,
            apns: None,
            fcm_options: None,
            target: self.target
        }
    }
}
