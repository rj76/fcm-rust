// cargo run --example simple_sender -- --help

use std::path::PathBuf;

use clap::Parser;
use fcm::{
    AndroidConfig, AndroidNotification, ApnsConfig, FcmClient, FcmOptions, Message, Notification, Target, WebpushConfig,
};
use serde_json::json;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(long)]
    device_token: String,
    /// Set path to the service account key JSON file. Default is to use
    /// path from the `GOOGLE_APPLICATION_CREDENTIALS` environment variable
    /// (which can be also located in `.env` file).
    #[arg(long, value_name = "FILE")]
    service_account_key_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = CliArgs::parse();
    let builder = FcmClient::builder();
    let builder = if let Some(path) = args.service_account_key_path {
        builder.service_account_key_json_path(path)
    } else {
        builder
    };

    let client = builder.build()
        .await
        .unwrap();

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
        target: Target::Token(args.device_token),
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
