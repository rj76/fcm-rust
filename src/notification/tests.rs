use crate::NotificationBuilder;
use serde_json::json;

#[test]
fn should_be_able_to_render_a_full_notification_to_json() {
    let mut builder = NotificationBuilder::new();

    builder
        .title("foo".to_string())
        .body("bar".to_string())
        .image("https://my.image.com/test.jpg".to_string());

    let payload = serde_json::to_string(&builder.finalize()).unwrap();

    let expected_payload = json!({
        "title": "foo",
        "body": "bar",
        "image": "https://my.image.com/test.jpg",
    })
    .to_string();

    assert_eq!(expected_payload, payload);
}

#[test]
fn should_set_notification_title() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title, None);

    let mut builder = NotificationBuilder::new();
    builder.title("title".to_string());
    let nm = builder.finalize();

    assert_eq!(nm.title, Some("title".to_string()));
}

#[test]
fn should_set_notification_body() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body, None);

    let mut builder = NotificationBuilder::new();
    builder.body("body".to_string());
    let nm = builder.finalize();

    assert_eq!(nm.body, Some("body".to_string()));
}

#[test]
fn should_set_notification_image() {
    let mut builder = NotificationBuilder::new();
    builder.image("https://my.image.com/test.jpg".to_string());
    let nm = builder.finalize();

    assert_eq!(nm.image, Some("https://my.image.com/test.jpg".to_string()));
}
