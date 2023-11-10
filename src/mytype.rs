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
#[derive(Serialize, Debug, Deserialize)]
pub struct GPTModel {
    id: String,
    name: String,
    #[serde(rename = "maxLength")]
    max_length: usize,
    #[serde(rename = "tokenLimit")]
    token_limit: usize,
}

impl GPTModel {
    pub fn default() -> GPTModel {
        GPTModel {
            id: String::from("openchat_v3.2_mistral"),
            name: String::from("OpenChat Aura"),
            max_length: 3000,
            token_limit: 65536,
        }
    }
}
#[derive(Serialize, Debug, Deserialize)]
pub struct GPTMessage {
    pub role: String,
    pub content: String,
}
impl GPTMessage {
    pub fn default(content: String) -> GPTMessage {
        GPTMessage {
            role: String::from("user"),
            content,
        }
    }
}
#[derive(Serialize, Debug, Deserialize)]
pub struct GPTRequestBody {
    model: GPTModel,
    prompt: String,
    temprature: f64,
    messages: Vec<GPTMessage>,
}
impl GPTRequestBody {
    pub fn new(model: GPTModel, messages: Vec<GPTMessage>) -> GPTRequestBody {
        GPTRequestBody {
            model,
            prompt: String::from(""),
            temprature: 0.5,
            messages,
        }
    }
}
