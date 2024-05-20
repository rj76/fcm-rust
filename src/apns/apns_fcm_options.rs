use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsfcmoptions
pub struct ApnsFcmOptions {
    /// Label associated with the message's analytics data.
    pub analytics_label: Option<String>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    pub image: Option<String>,
}
