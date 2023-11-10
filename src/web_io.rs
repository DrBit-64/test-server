use crate::mytype::*;
use hyper::{Body, Client, Method, Request};
use reqwest;
use serde_json::{self, Value};
use std::collections::HashMap;

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

pub async fn send_message_to_gpt(body: GPTRequestBody) -> String {
    let url = "https://openchat.team/api/chat";
    let body = serde_json::to_string(&body).unwrap();
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .body(body)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();
    response.text().await.unwrap()
}
