use crate::notification::NotificationBuilder;
use crate::{MessageBuilder, Target};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct CustomData {
    foo: &'static str,
    bar: bool,
}

#[test]
fn should_create_new_message() {
    let target = Target::Token("token".to_string());
    let msg = MessageBuilder::new(target.clone()).finalize();

    assert_eq!(msg.target, target);
}

#[test]
fn should_leave_nones_out_of_the_json() {
    let target = Target::Token("token".to_string());
    let msg = MessageBuilder::new(target).finalize();
    let payload = serde_json::to_string(&msg).unwrap();

    let expected_payload = json!({
        "target": "token"
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_add_custom_data_to_the_payload() {
    let target = Target::Token("token".to_string());
    let mut builder = MessageBuilder::new(target);

    let data = CustomData { foo: "bar", bar: false };

    builder.data(&data).unwrap();

    let msg = builder.finalize();
    let payload = serde_json::to_string(&msg).unwrap();

    let expected_payload = json!({
        "data": {
            "foo": "bar",
            "bar": false,
        },
        "target": "token"
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_be_able_to_render_a_full_message_to_json() {
    let target = Target::Token("token".to_string());
    let mut builder = MessageBuilder::new(target);

    builder.notification(NotificationBuilder::new().finalize());

    let payload = serde_json::to_string(&builder.finalize()).unwrap();

    let expected_payload = json!({
        "notification": {},
        "target": "token",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_set_notifications() {
    let target = Target::Token("token".to_string());
    let msg = MessageBuilder::new(target.clone()).finalize();

    assert_eq!(msg.notification, None);

    let nm = NotificationBuilder::new().finalize();

    let mut builder = MessageBuilder::new(target);
    builder.notification(nm);
    let msg = builder.finalize();

    assert_ne!(msg.notification, None);
}
