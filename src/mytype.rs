use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct GocqhttpError {
    message: String,
}

impl std::error::Error for GocqhttpError {}

impl Display for GocqhttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

impl GocqhttpError {
    pub fn new(message: String) -> GocqhttpError {
        GocqhttpError { message }
    }
}

#[derive(Serialize)]
pub struct Message {
    #[serde(rename = "type")]
    type_: String,
    data: HashMap<String, Value>,
}

impl Message {
    pub fn new(type_: String, data: HashMap<String, Value>) -> Message {
        Message { type_, data }
    }
}

#[derive(Serialize)]
pub struct PostBodySendGroupMsg {
    group_id: i64,
    message: Message,
}
