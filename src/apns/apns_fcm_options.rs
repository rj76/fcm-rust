use serde::Serialize;

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#apnsfcmoptions
pub(crate) struct ApnsFcmOptionsInternal {
    // Label associated with the message's analytics data.
    analytics_label: Option<String>,

    // Contains the URL of an image that is going to be displayed in a notification.
    image: Option<String>,
}

#[derive(Debug)]
pub struct ApnsFcmOptions {
    // Label associated with the message's analytics data.
    pub analytics_label: Option<String>,

    // Contains the URL of an image that is going to be displayed in a notification.
    pub image: Option<String>,
}

impl ApnsFcmOptions {
    pub(crate) fn finalize(self) -> ApnsFcmOptionsInternal {
        ApnsFcmOptionsInternal {
            analytics_label: self.analytics_label,
            image: self.image,
        }
    }
}
