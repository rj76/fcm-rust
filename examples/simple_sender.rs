// cargo run --example simple_sender -- -t <device_token>

use argparse::{ArgumentParser, Store};
use fcm::{Client, FcmOptions, Message, Notification, Target};
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

    let client = Client::new();

    let data = json!({
        "key": "value",
    });

    let builder = Message {
        data: Some(data),
        notification: Some(Notification {
            title: Some("Hello".to_string()),
            body: Some(format!("it's {}", chrono::Utc::now())),
            image: None,
        }),
        target: Target::Token(device_token),
        android: None,
        webpush: None,
        apns: None,
        fcm_options: Some(FcmOptions {
            analytics_label: "analytics_label".to_string(),
        }),
    };

    let response = client.send(builder).await?;
    println!("Sent: {:?}", response);

    Ok(())
}
