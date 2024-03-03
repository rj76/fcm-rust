use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#visibility
pub enum Visibility {
    VisibilityUnspecified,
    Private,
    Public,
    Secret,
}
