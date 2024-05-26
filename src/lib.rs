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
//!     use fcm::message::{Target, Notification, Message};
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
//!     let response = client.send(message).await?;
//!     println!("Response: {:?}", response);
//!
//!     Ok(())
//! }
//! ```

pub use yup_oauth2;

pub mod message;
pub(crate) mod notification;
pub(crate) mod android;
pub(crate) mod apns;
pub(crate) mod web;

mod client;
pub use crate::client::*;
