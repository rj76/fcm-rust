use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
//https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig
pub struct AndroidConfig {
    // An identifier of a group of messages that can be collapsed, so that only the last message gets
    // sent when delivery can be resumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    collapse_key: Option<String>,

    // Message priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<AndroidMessagePriority>,

    // How long (in seconds) the message should be kept in FCM storage if the device is offline.
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<String>,

    // Package name of the application where the registration token must match in order to receive the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_package_name: Option<String>,

    // Arbitrary key/value payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,

    // Notification to send to android devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification: Option<AndroidNotification>,

    // Options for features provided by the FCM SDK for Android.
    #[serde(skip_serializing_if = "Option::is_none")]
    fcm_options: Option<AndroidFcmOptions>,

    // If set to true, messages will be allowed to be delivered to the app while the device is in direct boot mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_boot_ok: Option<bool>
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#Color
pub struct Color {
    // The amount of red in the color as a value in the interval [0, 1].
    red: f32,

    // The amount of green in the color as a value in the interval [0, 1].
    green: f32,

    // The amount of blue in the color as a value in the interval [0, 1].
    blue: f32,

    // The fraction of this color that should be applied to the pixel.
    alpha: f32
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#LightSettings
pub struct LightSettings {
    // Set color of the LED with google.type.Color.
    color: Color,

    // Along with light_off_duration, define the blink rate of LED flashes
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    light_on_duration: String,

    // Along with light_on_duration, define the blink rate of LED flashes.
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    light_off_duration: String,
}

#[derive(Serialize, Debug)]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidnotification
pub struct AndroidNotification {
    // The notification's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    // The notification's body text.
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,

    // The notification's icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,

    // The notification's icon color, expressed in #rrggbb format.
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    // The sound to play when the device receives the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    sound: Option<String>,

    // Identifier used to replace existing notifications in the notification drawer.
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,

    // The action associated with a user click on the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    click_action: Option<String>,

    // The key to the body string in the app's string resources to use to localize the body text to the user's
    // current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_key: Option<String>,

    // Variable string values to be used in place of the format specifiers in body_loc_key to use to localize the
    // body text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    body_loc_args: Option<Vec<String>>,

    // The key to the title string in the app's string resources to use to localize the title text to the user's
    // current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_key: Option<String>,

    // Variable string values to be used in place of the format specifiers in title_loc_key to use to localize the
    // title text to the user's current localization.
    #[serde(skip_serializing_if = "Option::is_none")]
    title_loc_args: Option<Vec<String>>,

    // The notification's channel id (new in Android O).
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<String>,

    // Sets the "ticker" text, which is sent to accessibility services.
    #[serde(skip_serializing_if = "Option::is_none")]
    ticker: Option<String>,

    // When set to false or unset, the notification is automatically dismissed when the user clicks it in the panel.
    #[serde(skip_serializing_if = "Option::is_none")]
    sticky: Option<bool>,

    // Set the time that the event in the notification occurred. Notifications in the panel are sorted by this time.
    // Timestamp format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    event_time: Option<String>,

    // Set whether or not this notification is relevant only to the current device.
    #[serde(skip_serializing_if = "Option::is_none")]
    local_only: Option<bool>,

    // Set the relative priority for this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_priority: Option<NotificationPriority>,

    // If set to true, use the Android framework's default sound for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sound: Option<bool>,

    // If set to true, use the Android framework's default vibrate pattern for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_vibrate_timings: Option<bool>,

    // If set to true, use the Android framework's default LED light settings for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_light_settings: Option<bool>,

    // Set the vibration pattern to use
    // Duration format: https://developers.google.com/protocol-buffers/docs/reference/google.protobuf?authuser=0#google.protobuf.Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    vibrate_timings: Option<Vec<String>>,

    // Set the Notification.visibility of the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<Visibility>,

    // Sets the number of items this notification represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_count: Option<i32>,

    // Settings to control the notification's LED blinking rate and color if LED is available on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    light_settings: Option<LightSettings>,

    // Contains the URL of an image that is going to be displayed in a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
}

#[derive(Serialize, Debug)]
//https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidconfig
pub struct AndroidFcmOptions {
    // Label associated with the message's analytics data.
    analytics_label: String
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#androidmessagepriority
pub enum AndroidMessagePriority {
    Normal,
    High,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#notificationpriority
pub enum NotificationPriority {
    PriorityUnspecified,
    PriorityMin,
    PriorityLow,
    PriorityDefault,
    PriorityHigh,
    PriorityMax
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
// https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages?authuser=0#visibility
pub enum Visibility {
    VisibilityUnspecified,
    Private,
    Public,
    Secret
}
