use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug, Deserialize)]
pub struct FortuneData {
    pub text: String,
    pub result1: String,
    pub result2: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct FortuneState {
    // 1-5: 大凶 凶 中平 吉 大吉
    pub level: usize,
    pub index: Vec<usize>,
}
impl FortuneState {
    pub fn new(level: usize, index: Vec<usize>) -> FortuneState {
        FortuneState { level, index }
    }
}
