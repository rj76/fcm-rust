use serde::Serialize;

use super::color::{Color, ColorInternal};

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#LightSettings
pub struct LightSettingsInternal {
    // Set color of the LED with google.type.Color.
    color: ColorInternal,

    // Along with light_off_duration, define the blink rate of LED flashes
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    light_on_duration: String,

    // Along with light_on_duration, define the blink rate of LED flashes.
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    light_off_duration: String,
}

#[derive(Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#LightSettings
pub struct LightSettings {
    // Set color of the LED with google.type.Color.
    pub color: Color,

    // Along with light_off_duration, define the blink rate of LED flashes
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub light_on_duration: String,

    // Along with light_on_duration, define the blink rate of LED flashes.
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    pub light_off_duration: String,
}

impl LightSettings {
    pub fn finalize(self) -> LightSettingsInternal {
        LightSettingsInternal {
            color: self.color.finalize(),
            light_on_duration: self.light_on_duration,
            light_off_duration: self.light_off_duration,
        }
    }
}
