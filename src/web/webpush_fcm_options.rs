use serde::Serialize;

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushfcmoptions
pub struct WebpushFcmOptionsInternal {
    // The link to open when the user clicks on the notification.
    link: String,

    // Label associated with the message's analytics data.
    analytics_label: String,
}

#[derive(Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#webpushfcmoptions
pub struct WebpushFcmOptions {
    // The link to open when the user clicks on the notification.
    pub link: String,

    // Label associated with the message's analytics data.
    pub analytics_label: String,
}

impl WebpushFcmOptions {
    pub fn finalize(self) -> WebpushFcmOptionsInternal {
        WebpushFcmOptionsInternal {
            link: self.link,
            analytics_label: self.analytics_label,
        }
    }
}
