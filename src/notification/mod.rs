#[cfg(test)]
mod tests;

use serde::Serialize;

/// This struct represents a FCM notification. Use the
/// corresponding `Notification` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#notification
pub(crate) struct NotificationInternal {
    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    /// The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,

    /// Contains the URL of an image that is going to be downloaded on the device and displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

#[derive(Debug, Default)]
pub struct Notification {
    /// The notification's title.
    pub title: Option<String>,

    /// The notification's body text.
    pub body: Option<String>,

    /// Contains the URL of an image that is going to be downloaded on the device and displayed in a notification.
    pub image: Option<String>,
}

impl Notification {
    /// Complete the build and get a `Notification` instance
    pub(crate) fn finalize(self) -> NotificationInternal {
        NotificationInternal {
            title: self.title,
            body: self.body,
            image: self.image,
        }
    }
}
