use serde::Serialize;

use super::{light_settings::LightSettings, notification_priority::NotificationPriority, visibility::Visibility};

#[derive(Debug, Default, Serialize)]
/// <https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidnotification>
pub struct AndroidNotification {
    /// The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// The notification's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The notification's icon color, expressed in #rrggbb format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// The sound to play when the device receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,

    /// Identifier used to replace existing notifications in the notification drawer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// The action associated with a user click on the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_action: Option<String>,

    /// The key to the body string in the app's string resources to use to localize the body text to the user's
    /// current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_loc_key: Option<String>,

    /// Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the
    /// body text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_loc_args: Option<Vec<String>>,

    /// The key to the title string in the app's string resources to use to localize the title text to the user's
    /// current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_key: Option<String>,

    /// Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the
    /// title text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_loc_args: Option<Vec<String>>,

    /// The notification's channel id (new in Android O).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    /// Sets the "ticker" text, which is sent to accessibility services.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,

    /// When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,

    /// Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time.
    /// Timestamp format: <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Timestamp>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,

    /// Set whether or not this notification is relevant only to the current device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_only: Option<bool>,

    /// Set the relative priority for this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_priority: Option<NotificationPriority>,

    /// If set to true, use the Android framework's default sound for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sound: Option<bool>,

    /// If set to true, use the Android framework's default vibrate pattern for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_vibrate_timings: Option<bool>,

    /// If set to true, use the Android framework's default LED light settings for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_light_settings: Option<bool>,

    /// Set the vibration pattern to use
    /// Duration format: <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vibrate_timings: Option<Vec<String>>,

    /// Set the Notification.visibility of the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    /// Sets the number of items this notification represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_count: Option<i32>,

    /// Settings to control the notification's LED blinking rate and color if LED is available on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_settings: Option<LightSettings>,

    /// Contains the URL of an image that is going to be displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
