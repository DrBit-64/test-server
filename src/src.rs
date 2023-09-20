use hyper::body::Bytes;
use hyper::{Body, Client, Method, Request};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::io::Read;

pub mod utils;
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
            let total_path = format!("./data/total{}", group_id);
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

async fn get_group_member_name(
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

async fn produce_daily_report_message(
    group_id: i64,
) -> Result<Message, Box<dyn std::error::Error>> {
    let mut message_str = String::from("以下为今天的水群榜：");
    let file_path = format!("./data/daily/{}.json", group_id);
    let data = read_json_file(&file_path)?;
    let mut sorted_vec: Vec<(&String, &i64)> = data.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (idx, (k, v)) in sorted_vec.iter().enumerate() {
        let user_id: i64 = k.parse()?;
        let name = get_group_member_name(group_id, user_id).await?;
        message_str = format!("{}\n{}.{}: {}", message_str, idx + 1, name, v);
    }
    Ok(convert_string_to_message(message_str))
}

async fn produce_total_report_message(
    group_id: i64,
) -> Result<Message, Box<dyn std::error::Error>> {
    let mut message_str = String::from("以下为水群总榜：");
    let total_file_path = format!("./data/total/{}.json", group_id);
    let mut total_data = read_json_file(&total_file_path)?;
    let daily_file_path = format!("./data/daily/{}.json", group_id);
    let daily_data = read_json_file(&daily_file_path)?;
    for (k, v) in daily_data.into_iter() {
        let counter = total_data.entry(k).or_insert(0);
        *counter += v;
    }
    let mut sorted_vec: Vec<(&String, &i64)> = total_data.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (idx, (k, v)) in sorted_vec.iter().enumerate() {
        let user_id: i64 = k.parse()?;
        let name = get_group_member_name(group_id, user_id).await?;
        message_str = format!("{}\n{}.{}: {}", message_str, idx + 1, name, v);
    }
    Ok(convert_string_to_message(message_str))
}

fn convert_string_to_message(s: String) -> Message {
    let mut data: HashMap<String, Value> = HashMap::new();
    data.insert(String::from("text"), Value::String(s));
    Message::new(String::from("text"), data)
}

fn get_pet_list() -> Result<String, Box<dyn std::error::Error>> {
    let cur_dir = std::env::current_dir()?;
    let grandparent_dir = cur_dir.parent().unwrap().parent().unwrap();
    let pet_dir = grandparent_dir
        .join("petpet")
        .join("data")
        .join("xmmt.dituon.petpet");
    let mut pet_list = String::from("以下为可用的petpet指令");
    if let Ok(entries) = fs::read_dir(pet_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let folder_name = entry.file_name().to_str().unwrap_or("").to_string();
                if folder_name == "fonts" {
                    continue;
                }
                let path = entry.path();
                let data_path = path.join("data.json");
                let contents = fs::read_to_string(data_path)?;
                let data: HashMap<String, Value> = serde_json::from_str(&contents)?;
                // println!("data:{:?}\n", data);
                // let names = data.get("alias").unwrap().as_array().unwrap();
                if let Some(tmp) = data.get("alias") {
                    let names = tmp.as_array().unwrap();
                    pet_list = format!("{}\n{} ", pet_list, folder_name);
                    for name in names {
                        pet_list = format!("{} {}", pet_list, name.as_str().unwrap());
                    }
                }
            }
        }
        return Ok(pet_list);
    }
    Ok(String::from(""))
}

fn add_cnt(group_id: i64, user_id: i64) {
    let file_path = format!("./data/daily/{}.json", group_id);
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

pub async fn analyze_post_body(body: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    let body: HashMap<String, Value> =
        serde_json::from_slice(&body).expect("error when convert body to Hashmap");
    if let Some(post_type) = body.get("post_type") {
        if post_type == "message" && body.get("message_type").unwrap() == "group" {
            let group_id = body.get("group_id").unwrap().as_i64().unwrap();
            let user_id = body.get("user_id").unwrap().as_i64().unwrap();
            let raw_message = body.get("raw_message").unwrap().as_str().unwrap();
            match raw_message {
                "!!ping" => send_string_to_group(String::from("pong!!"), group_id).await?,
                "!!daily rank" => {
                    let message = produce_daily_report_message(group_id).await?;
                    send_message_to_group(message, group_id).await?
                }
                "!!total rank" => {
                    let message = produce_total_report_message(group_id).await?;
                    send_message_to_group(message, group_id).await?;
                }
                "!!petpet list" => {
                    let pet_list = get_pet_list()?;
                    send_string_to_group(pet_list, group_id).await?;
                }
                _ => add_cnt(group_id, user_id),
            }
        }
    }
    Ok(())
}

pub async fn daily_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = open_or_create_file("./target.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let groups: Vec<i64> = serde_json::from_str(&contents)?;
    println!("groups:{:?}", groups);
    for group_id in groups {
        println!("group:{}", group_id);
        let message = produce_daily_report_message(group_id).await?;
        send_message_to_group(message, group_id).await?;
    }
    transfer_data();
    Ok(())
}
