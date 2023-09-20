use crate::mytype::*;
use hyper::{Body, Client, Method, Request};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

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

pub async fn send_messages_to_group(
    messages: Vec<Message>,
    group_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut post_body: HashMap<String, Value> = HashMap::new();
    let messages = serde_json::to_value(messages)?;
    let group_id = serde_json::to_value(group_id)?;
    post_body.insert("message".to_string(), messages);
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

pub async fn send_message_to_group(
    message: Message,
    group_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut messages = Vec::new();
    messages.push(message);
    send_messages_to_group(messages, group_id).await
}

pub async fn send_string_to_group(
    message_str: String,
    group_id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data: HashMap<String, Value> = HashMap::new();
    data.insert(String::from("text"), Value::String(message_str));
    let message = Message::new(String::from("text"), data);
    send_message_to_group(message, group_id).await
}

pub fn clear_all_wife_data() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir("./data/wife")?;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let file_path = format!("./data/wife/{}", file_name);
                let mut data = read_json_file(&file_path)?;
                data.clear();
                write_data_to_file(&data, &file_path)?;
            }
        }
    }
    Ok(())
}
