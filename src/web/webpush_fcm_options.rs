use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushfcmoptions
pub struct WebpushFcmOptions {
    /// The link to open when the user clicks on the notification.
    pub link: String,

    /// Label associated with the message's analytics data.
    pub analytics_label: String,
}
