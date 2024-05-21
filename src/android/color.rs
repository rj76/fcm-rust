use serde::Serialize;

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#Color>
pub struct Color {
    /// The amount of red in the color as a value in the interval [0, 1].
    pub red: f32,

    /// The amount of green in the color as a value in the interval [0, 1].
    pub green: f32,

    /// The amount of blue in the color as a value in the interval [0, 1].
    pub blue: f32,

    /// The fraction of this color that should be applied to the pixel.
    pub alpha: f32,
}
