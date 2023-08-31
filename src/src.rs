use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::io::{Read, Write};
use std::{collections::HashMap, net::TcpStream};
pub mod data;
pub mod utils;

fn analyze_post_body(body: &str) {
    let body: HashMap<String, Value> =
        serde_json::from_str(body).expect("error when convert body to Hashmap");
    if let Some(post_type) = body.get("post_type") {
        if post_type == "message" && body.get("message_type").unwrap() == "group" {
            let group_id = body.get("group_id").unwrap().as_i64().unwrap();
            let user_id = body.get("user_id").unwrap().as_i64().unwrap();
            data::add_cnt(group_id, user_id);
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        request.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    if !request.starts_with("POST / HTTP/1.1") {
        return;
    }
    let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
    let body = &request[body_start..];
    analyze_post_body(body);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn delayed_sending() {
    todo!()
}
