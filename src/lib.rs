//! fcm
//! ===
//!
//! A client for asynchronous sending of Firebase Cloud Messages, or Push Notifications.
//!
//! # Examples
//!
//! To send out a FCM Message with some custom data:
//!
//! ```no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     use serde_json::json;
//!     use fcm::{Target, FcmOptions, Notification, Message};
//!     let client = fcm::FcmClient::builder()
//!         // Comment to use GOOGLE_APPLICATION_CREDENTIALS environment
//!         // variable. The variable can also be defined in .env file.
//!         .service_account_key_json_path("service_account_key.json")
//!         .build()
//!         .await
//!         .unwrap();
//!
//!     // Replace "device_token" with the actual device token
//!     let device_token = "device_token".to_string();
//!     let message = Message {
//!         data: Some(json!({
//!            "message": "Howdy!",
//!         })),
//!         notification: Some(Notification {
//!             title: Some("Hello".to_string()),
//!             body: Some(format!("it's {}", chrono::Utc::now())),
//!             image: None,
//!         }),
//!         target: Target::Token("device_token".to_string()),
//!         android: None,
//!         webpush: None,
//!         apns: None,
//!         fcm_options: None,
//!     };
//!
//!     let response = client.send(builder).await?;
//!     println!("Response: {:?}", response);
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
pub use crate::client::response::*;
pub use crate::client::*;
