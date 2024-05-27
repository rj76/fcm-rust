use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#fcmoptions>
pub struct FcmOptions {
    /// Label associated with the message's analytics data.
    pub analytics_label: String,
}
