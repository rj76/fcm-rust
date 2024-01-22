#[cfg(test)]
mod tests;

use serde::Serialize;

/// This struct represents a FCM notification. Use the
/// corresponding `NotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#notification
pub(crate) struct NotificationInternal {
    // The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    // The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,

    // Contains the URL of an image that is going to be downloaded on the device and displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

/// A builder to get a `Notification` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::NotificationBuilder;
///
/// let mut builder = NotificationBuilder::new();
///  builder.title("Australia vs New Zealand".to_string());
/// builder.body("3 runs to win in 1 ball".to_string());
/// let notification = builder.finalize();
/// ```
#[derive(Debug)]
pub struct Notification {
    pub title: Option<String>,
    pub body: Option<String>,
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
