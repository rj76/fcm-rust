use serde::Serialize;

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#fcmoptions
pub struct FcmOptionsInternal {
    // Label associated with the message's analytics data.
    analytics_label: String,
}

#[derive(Debug)]
pub struct FcmOptions {
    pub analytics_label: String,
}

impl FcmOptions {
    pub fn finalize(self) -> FcmOptionsInternal {
        FcmOptionsInternal {
            analytics_label: self.analytics_label,
        }
    }
}
