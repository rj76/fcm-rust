#[cfg(test)]
mod tests;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#notification>
pub struct Notification {
    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Contains the URL of an image that is going to be downloaded on the device and displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
