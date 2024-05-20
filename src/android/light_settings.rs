use serde::Serialize;

use super::color::Color;

#[derive(Debug, Default, Serialize)]
/// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#LightSettings
pub struct LightSettings {
    /// Set color of the LED with google.type.Color.
    pub color: Color,

    /// Along with light_off_duration, define the blink rate of LED flashes
    /// Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub light_on_duration: String,

    /// Along with light_on_duration, define the blink rate of LED flashes.
    /// Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub light_off_duration: String,
}
