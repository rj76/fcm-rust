use serde::Serialize;

#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Token(String),
    Topic(String),
    Condition(String),
}
