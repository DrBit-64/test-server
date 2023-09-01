use hyper::body::Bytes;
use hyper::{Body, Client, Method, Request};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs;

mod utils;
use utils::*;

#[derive(Debug)]
struct GocqhttpError {
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

fn transfer_daily_to_total(path: &str) {
    match read_json_file(path) {
        Ok(mut daily_data) => {
            let group_id = remove_prefix(path, "./data/daily"); //with ".json"
            let total_path = "./data/total".to_owned() + group_id;
            let mut total_data: HashMap<String, i64> =
                read_json_file(&total_path).expect("total data file open failed");
            for (k, v) in &daily_data {
                let counter = total_data.entry(k.to_string()).or_insert(0);
                *counter += v;
            }
            daily_data.clear();
            if let Err(err) = write_data_to_file(&daily_data, path) {
                println!("fail when write data to file {}:{}", path, err);
            }
            if let Err(err) = write_data_to_file(&total_data, &total_path) {
                println!("fail when write data to file {}:{}", path, err);
            }
        }
        Err(err) => {
            println!("error when transfer daily data to total:{}", err);
        }
    }
}

fn transfer_data() {
    if let Ok(entries) = fs::read_dir("./data/daily") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    // 处理文件
                    transfer_daily_to_total(path.to_str().unwrap());
                }
            }
        }
    }
}

pub async fn get_group_member_name(
    group_id: i64,
    user_id: i64,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut post_body: HashMap<String, i64> = HashMap::new();
    post_body.insert("group_id".to_string(), group_id);
    post_body.insert("user_id".to_string(), user_id);
    let json_data = serde_json::to_string(&post_body)?;

    let client = Client::new();
    let request = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5700/get_group_member_info")
        .header("Content-Type", "application/json")
        .body(Body::from(json_data))
        .unwrap();
    let response = client.request(request).await?;
    if response.status().is_client_error() {
        return Err(Box::new(GocqhttpError::new(
            "get group member info request failed".to_string(),
        )));
    }
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body_bytes.to_vec())?;
    let json_value: Value = serde_json::from_str(&body_str)?;
    let mut body: HashMap<String, Value> = serde_json::from_value(json_value)?;
    let json_data: Value = body.remove("data").unwrap();
    let member_info: HashMap<String, Value> = serde_json::from_value(json_data)?;
    println!("{:?}", member_info);

    if let Some(card) = member_info.get("card").and_then(|v| v.as_str()) {
        if !card.is_empty() {
            return Ok(card.to_string());
        }
    }
    if let Some(nickname) = member_info.get("nickname").and_then(|v| v.as_str()) {
        return Ok(nickname.to_string());
    }
    return Ok("".to_string());
}

fn add_cnt(group_id: i64, user_id: i64) {
    let file_path = "./data/daily/".to_owned() + &group_id.to_string() + ".json";
    match read_json_file(&file_path) {
        Ok(mut data) => {
            let counter = data.entry(user_id.to_string()).or_insert(0);
            *counter += 1;
            if let Err(err) = write_data_to_file(&data, &file_path) {
                println!("error when write json to file {}: {}", &file_path, err)
            }
        }
        Err(err) => println!("error when read json from daily file {}", err),
    }
}

pub fn analyze_post_body(body: Bytes) {
    let body: HashMap<String, Value> =
        serde_json::from_slice(&body).expect("error when convert body to Hashmap");
    if let Some(post_type) = body.get("post_type") {
        if post_type == "message" && body.get("message_type").unwrap() == "group" {
            let group_id = body.get("group_id").unwrap().as_i64().unwrap();
            let user_id = body.get("user_id").unwrap().as_i64().unwrap();
            add_cnt(group_id, user_id);
        }
    }
}

pub fn delayed_sending() {
    todo!()
}
