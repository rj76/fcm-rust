//! fcm
//! ===
//!
//! A client for asynchronous sending of Firebase Cloud Messages, or Push Notifications.
//!
//! # Examples:
//!
//! To send out a FCM Message with some custom data:
//!
//! ```no_run
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     use serde_json::json;
//!     use fcm::{Target, FcmOptions, Notification, Message};
//!     let client = fcm::Client::new();
//!
//!     let data = json!({
//!         "message": "Howdy!"
//!     });
//!
//!     let builder = Message {
//!         data: Some(data),
//!         notification: Some(Notification {
//!             title: Some("Hello".to_string()),
//!             body: Some(format!("it's {}", chrono::Utc::now())),
//!             image: None,
//!         }),
//!         target: Target::Token("token".to_string()),
//!         android: None,
//!         webpush: None,
//!         apns: None,
//!         fcm_options: Some(FcmOptions {
//!             analytics_label: "analytics_label".to_string(),
//!         }),
//!     };
//!
//!     let response = client.send(builder).await?;
//!     println!("Sent: {:?}", response);
//!
//!     Ok(())
//! }
//! ```

mod message;
pub use crate::message::fcm_options::*;
pub use crate::message::target::*;
pub use crate::message::*;

mod notification;
pub use crate::notification::*;

mod android;
pub use crate::android::android_config::*;
pub use crate::android::android_fcm_options::*;
pub use crate::android::android_message_priority::*;
pub use crate::android::android_notification::*;
pub use crate::android::color::*;
pub use crate::android::light_settings::*;
pub use crate::android::notification_priority::*;
pub use crate::android::visibility::*;

mod apns;
pub use crate::apns::apns_config::*;
pub use crate::apns::apns_fcm_options::*;

mod web;
pub use crate::web::webpush_config::*;
pub use crate::web::webpush_fcm_options::*;

mod client;
pub use crate::client::response::FcmError as Error;
pub use crate::client::*;
