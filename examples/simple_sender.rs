// cargo run --example simple_sender -- -t <device_token>

use argparse::{ArgumentParser, Store};
use fcm::{
    AndroidConfig, AndroidNotification, ApnsConfig, Client, FcmOptions, Message, Notification, Target, WebpushConfig,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let mut device_token = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A simple FCM notification sender");
        ap.refer(&mut device_token)
            .add_option(&["-t", "--device_token"], Store, "Device token");
        ap.parse_args_or_exit();
    }

    let client = Client::new("service-account-key.json".to_string());

    let data = json!({
        "key": "value",
    });

    let builder = Message {
        data: Some(data),
        notification: Some(Notification {
            title: Some("I'm high".to_string()),
            body: Some(format!("it's {}", chrono::Utc::now())),
            ..Default::default()
        }),
        target: Target::Token(device_token),
        fcm_options: Some(FcmOptions {
            analytics_label: "analytics_label".to_string(),
        }),
        android: Some(AndroidConfig {
            priority: Some(fcm::AndroidMessagePriority::High),
            notification: Some(AndroidNotification {
                title: Some("I'm Android high".to_string()),
                body: Some(format!("Hi Android, it's {}", chrono::Utc::now())),
                ..Default::default()
            }),
            ..Default::default()
        }),
        apns: Some(ApnsConfig { ..Default::default() }),
        webpush: Some(WebpushConfig { ..Default::default() }),
    };

    let response = client.send(builder).await?;
    println!("Sent: {:?}", response);

    Ok(())
}
