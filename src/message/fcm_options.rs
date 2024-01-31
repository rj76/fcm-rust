use serde::Serialize;

#[derive(Serialize, Debug)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#fcmoptions
pub(crate) struct FcmOptionsInternal {
    /// Label associated with the message's analytics data.
    analytics_label: String,
}

#[derive(Debug, Default)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#fcmoptions
pub struct FcmOptions {
    /// Label associated with the message's analytics data.
    pub analytics_label: String,
}

impl FcmOptions {
    pub(crate) fn finalize(self) -> FcmOptionsInternal {
        FcmOptionsInternal {
            analytics_label: self.analytics_label,
        }
    }
}
