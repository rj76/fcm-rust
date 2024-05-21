// cargo run --example simple_sender -- --help

use std::path::PathBuf;

use clap::Parser;
use fcm::{
    FcmClient, Message, Notification, Target,
};
use serde_json::json;

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short = 't', long)]
    device_token: String,
    /// Set path to the service account key JSON file. Default is to use
    /// path from the `GOOGLE_APPLICATION_CREDENTIALS` environment variable
    /// (which can be also located in `.env` file).
    #[arg(short = 'k', long, value_name = "FILE")]
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

    let message = Message {
        data: Some(json!({
            "key": "value",
        })),
        notification: Some(Notification {
            title: Some("Title".to_string()),
            ..Default::default()
        }),
        target: Target::Token(args.device_token),
        fcm_options: None,
        android: None,
        apns: None,
        webpush: None,
    };

    let response = client.send(message).await?;
    println!("Response: {:#?}", response);

    Ok(())
}
