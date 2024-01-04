# fcm-rust
[![Cargo tests](https://github.com/rj76/fcm-rust/actions/workflows/test.yml/badge.svg)](https://github.com/rj76/fcm-rust/actions/workflows/test.yml)
[![Coverage Status](https://coveralls.io/repos/github/rj76/fcm-rust/badge.svg)](https://coveralls.io/github/rj76/fcm-rust)

[//]: # ([![Crates.io Version]&#40;https://img.shields.io/crates/v/fcm.svg?style=flat-square&#41;)
[//]: # ([![Crates.io Downloads]&#40;https://img.shields.io/crates/dv/fcm.svg?style=flat-square&#41;)
[//]: # ([![Crates.io License]&#40;https://img.shields.io/crates/l/fcm.svg?style=flat-square&#41;)


## v1 API

This fork is a rewrite to use Google's HTTP v1 API.

# Credentials

This library expects the Google credentials JSON location to be 
defined as `GOOGLE_APPLICATION_CREDENTIALS` in the `.env` file.
Please follow the instructions in the [Firebase Documentation](https://firebase.google.com/docs/cloud-messaging/auth-server#provide-credentials-manually) to create a service account.

## Examples

Check out the examples directory for a simple sender.
