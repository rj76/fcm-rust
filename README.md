# fcm-rust
[![Cargo tests](https://github.com/rj76/fcm-rust/actions/workflows/test.yml/badge.svg)](https://github.com/rj76/fcm-rust/actions/workflows/test.yml)
[![Coverage Status](https://coveralls.io/repos/github/rj76/fcm-rust/badge.svg)](https://coveralls.io/github/rj76/fcm-rust)

[//]: # ([![Crates.io Version]&#40;https://img.shields.io/crates/v/fcm.svg?style=flat-square&#41;)
[//]: # ([![Crates.io Downloads]&#40;https://img.shields.io/crates/dv/fcm.svg?style=flat-square&#41;)
[//]: # ([![Crates.io License]&#40;https://img.shields.io/crates/l/fcm.svg?style=flat-square&#41;)


## v1 API

This fork is a rewrite to use Google's HTTP v1 API.


# Getting started

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
fcm = { git = "https://github.com/rj76/fcm-rust.git" }
```

Then, you need to add the credentials described in the [Credentials](#credentials) to a `.env` file at the root of your project.

## Usage

For a complete usage example, you may check the [Examples](#examples) section.

### Import

```rust
use fcm;
```

### Create a client instance

```rust
let client = fcm::Client::new();
```

### Construct a message

```rust
let message = fcm::Message {
    data: None,
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
}
```

### Send the message

```rust
let response = client.send(message).await?;
```

# Credentials

This library expects the Google credentials JSON location to be 
defined as `GOOGLE_APPLICATION_CREDENTIALS` in the `.env` file.
Please follow the instructions in the [Firebase Documentation](https://firebase.google.com/docs/cloud-messaging/auth-server#provide-credentials-manually) to create a service account.

## Examples

For a complete usage example, you may check out the [`simple_sender`](examples/simple_sender.rs) example.

To run the example, first of all clone the [`.env.example`](.env.example) file to `.env` and fill in the required values.

You can find info about the required credentials in the [Credentials](#credentials) section.

Then run the example with `cargo run --example simple_sender -- -t <device_token>`



