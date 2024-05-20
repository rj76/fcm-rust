use serde::Serialize;

/// Target to send a message to.
///
/// ```rust
/// use fcm::{Target};
///
/// Target::Token("myfcmtoken".to_string());
/// Target::Topic("my-topic-name".to_string());
/// Target::Condition("my-condition".to_string());
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Token(String),
    Topic(String),
    Condition(String),
}
