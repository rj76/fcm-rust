use serde::Serialize;

#[derive(Serialize, Debug)]
//https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig
pub(crate) struct AndroidFcmOptionsInternal {
    // Label associated with the message's analytics data.
    analytics_label: String,
}

#[derive(Debug)]
pub struct AndroidFcmOptions {
    pub analytics_label: String,
}

impl AndroidFcmOptions {
    pub(crate) fn finalize(self) -> AndroidFcmOptionsInternal {
        AndroidFcmOptionsInternal {
            analytics_label: self.analytics_label,
        }
    }
}
