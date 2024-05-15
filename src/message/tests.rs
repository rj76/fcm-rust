use crate::{message::Target, notification::Notification, Message};
use serde_json::json;

#[test]
fn should_create_new_message() {
    let target = Target::Token("token".to_string());
    let msg = Message {
        target: target.clone(),
        data: None,
        notification: None,
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    }
    .finalize();

    assert_eq!(msg.target, target);
}

#[test]
fn should_leave_nones_out_of_the_json() {
    let target = Target::Token("token".to_string());
    let msg = Message {
        target: target.clone(),
        data: None,
        notification: None,
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    }
    .finalize();
    let payload = serde_json::to_string(&msg).unwrap();

    let expected_payload = json!({
        "token": "token"
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_add_custom_data_to_the_payload() {
    let target = Target::Token("token".to_string());
    let data = json!({ "foo": "bar", "bar": false });

    let builder = Message {
        target,
        data: Some(data),
        notification: None,
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    };

    let msg = builder.finalize();
    let payload = serde_json::to_string(&msg).unwrap();

    let expected_payload = json!({
        "data": {
            "foo": "bar",
            "bar": false,
        },
        "token": "token"
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_be_able_to_render_a_full_token_message_to_json() {
    let target = Target::Token("token".to_string());
    let notification = Notification {
        title: None,
        body: None,
        image: None,
    };
    let builder = Message {
        target: target.clone(),
        data: None,
        notification: Some(notification),
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    };

    let payload = serde_json::to_string(&builder.finalize()).unwrap();

    let expected_payload = json!({
        "notification": {},
        "token": "token",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_be_able_to_render_a_full_topic_message_to_json() {
    let target = Target::Topic("my_topic".to_string());
    let notification = Notification {
        title: None,
        body: None,
        image: None,
    };
    let builder = Message {
        target: target.clone(),
        data: None,
        notification: Some(notification),
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    };

    let payload = serde_json::to_string(&builder.finalize()).unwrap();

    let expected_payload = json!({
        "notification": {},
        "topic": "my_topic",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_be_able_to_render_a_full_condition_message_to_json() {
    let target = Target::Condition("my_condition".to_string());
    let notification = Notification {
        title: None,
        body: None,
        image: None,
    };
    let builder = Message {
        target: target.clone(),
        data: None,
        notification: Some(notification),
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    };

    let payload = serde_json::to_string(&builder.finalize()).unwrap();

    let expected_payload = json!({
        "notification": {},
        "condition": "my_condition",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_set_notifications() {
    let target = Target::Token("token".to_string());

    let nm = Notification {
        title: None,
        body: None,
        image: None,
    };

    let builder = Message {
        target: target.clone(),
        data: None,
        notification: Some(nm),
        android: None,
        webpush: None,
        apns: None,
        fcm_options: None,
    };
    let msg = builder.finalize();

    assert!(msg.notification.is_some());
}
