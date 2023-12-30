use serde::Serialize;

#[cfg(test)]
mod tests;

/// This struct represents a FCM notification. Use the
/// corresponding `NotificationBuilder` to get an instance. You can then use
/// this notification instance when sending a FCM message.
#[derive(Serialize, Debug, PartialEq)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#notification
pub struct Notification {
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
#[derive(Default)]
pub struct NotificationBuilder {
    title: Option<String>,
    body: Option<String>,
    image: Option<String>,
}

impl NotificationBuilder {
    /// Get a new `NotificationBuilder` instance, with a title.
    pub fn new() -> NotificationBuilder {
        Self::default()
    }

    // Set the title of the notification
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Set the body of the notification
    pub fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    /// Set the image
    pub fn image(&mut self, image: String) -> &mut Self {
        self.image = Some(image);
        self
    }

    /// Complete the build and get a `Notification` instance
    pub fn finalize(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            image: self.image
        }
    }
}
