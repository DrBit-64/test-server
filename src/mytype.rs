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

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: HashMap<String, Value>,
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
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}
impl ChatMessage {
    pub fn default(content: String) -> ChatMessage {
        ChatMessage {
            role: String::from("user"),
            content,
        }
    }
    pub fn new(role: String, content: String) -> ChatMessage {
        ChatMessage { role, content }
    }
}
#[derive(Serialize, Debug, Deserialize)]
pub struct GPTRequestBody {
    model: GPTModel,
    prompt: String,
    temprature: f64,
    messages: Vec<ChatMessage>,
}
impl GPTRequestBody {
    pub fn new(model: GPTModel, messages: Vec<ChatMessage>) -> GPTRequestBody {
        GPTRequestBody {
            model,
            prompt: String::from(""),
            temprature: 0.5,
            messages,
        }
    }
}
