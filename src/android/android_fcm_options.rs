use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct AndroidFcmOptionsInternal {
    analytics_label: String,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig
pub struct AndroidFcmOptions {
    /// Label associated with the message's analytics data.
    pub analytics_label: String,
}

impl AndroidFcmOptions {
    pub(crate) fn finalize(self) -> AndroidFcmOptionsInternal {
        AndroidFcmOptionsInternal {
            analytics_label: self.analytics_label,
        }
    }
}
