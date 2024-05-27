# fcm-rust
[![Cargo tests](https://github.com/rj76/fcm-rust/actions/workflows/test.yml/badge.svg)](https://github.com/rj76/fcm-rust/actions/workflows/test.yml)

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

Optionally, add the credentials described in the [Credentials](#credentials)
to a `.env` file at the root of your project.

## Usage

For a complete usage example, you may check the [Examples](#examples) section.

### Import

```rust
use fcm;
```

### Create a client instance

```rust
let client = fcm::FcmClient::builder()
    // Comment to use GOOGLE_APPLICATION_CREDENTIALS environment
    // variable. The variable can also be defined in .env file.
    .service_account_key_json_path("service_account_key.json")
    .build()
    .await
    .unwrap();
```

### Construct a message

```rust
use fcm::message::{Message, Notification, Target};

// Replace "device_token" with the actual device token
let device_token = "device_token".to_string();
let message = Message {
    data: Some(json!({
       "message": "Howdy!",
    })),
    notification: Some(Notification {
        title: Some("Hello".to_string()),
        body: Some(format!("it's {}", chrono::Utc::now())),
        image: None,
    }),
    target: Target::Token(device_token),
    android: None,
    webpush: None,
    apns: None,
    fcm_options: None,
};
```

### Send the message

```rust
let response = client.send(message).await.unwrap();
```

# Credentials

If client is not configured with service account key JSON file path
then this library expects the Google credentials JSON location to be
defined in `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
The variable definition can also be located in the `.env` file.

Please follow the instructions in the
[Firebase Documentation](https://firebase.google.com/docs/cloud-messaging/auth-server#provide-credentials-manually)
to create a service account key JSON file.

## Examples

For a complete usage example, you may check out the
[`simple_sender`](examples/simple_sender.rs) example.

The example can be run with
```
cargo run --example simple_sender -- -t <device_token> -k <service_account_key_path>
```

If `GOOGLE_APPLICATION_CREDENTIALS` environment variable is defined in current
environment or in `.env` file, then the example can be run with
```
cargo run --example simple_sender -- -t <device_token>
```

To define the environment variable using `.env` file copy the [`.env.example`](.env.example)
file to `.env` and fill in the required values.
