use hyper::{Body, Client, Method, Request};
use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

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

pub fn remove_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    if input.starts_with(prefix) {
        // 使用字符串切片去除前缀
        &input[prefix.len()..]
    } else {
        input
    }
}

pub fn open_or_create_file(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

pub fn read_json_file(file_path: &str) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let mut file = open_or_create_file(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let data: HashMap<String, i64> = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn write_data_to_file(data: &HashMap<String, i64>, file_path: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let mut file = File::create(file_path)?;
    file.set_len(0)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub async fn send_message_to_group(
    message: Message,
    group_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut post_body: HashMap<String, Value> = HashMap::new();
    let message = serde_json::to_value(message)?;
    let group_id = serde_json::to_value(group_id)?;
    post_body.insert("message".to_string(), message);
    post_body.insert("group_id".to_string(), group_id);
    let json_data = serde_json::to_string(&post_body)?;

    let client = Client::new();
    let request = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5700/send_group_msg")
        .header("Content-Type", "application/json")
        .body(Body::from(json_data))?;
    let response = client.request(request).await?;
    if response.status().is_client_error() {
        println!("error when send group msg:{}", response.status());
    }
    Ok(())
}
