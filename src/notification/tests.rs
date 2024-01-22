use crate::Notification;
use serde_json::json;

#[test]
fn should_be_able_to_render_a_full_notification_to_json() {
    let not = Notification {
        title: Some("foo".to_string()),
        body: Some("bar".to_string()),
        image: Some("https://my.image.com/test.jpg".to_string()),
    };

    let payload = serde_json::to_string(&not.finalize()).unwrap();

    let expected_payload = json!({
        "title": "foo",
        "body": "bar",
        "image": "https://my.image.com/test.jpg",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}
