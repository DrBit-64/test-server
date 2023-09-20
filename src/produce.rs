use crate::utils::*;
use hyper::{Body, Client, Method, Request};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs;

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

pub async fn produce_daily_report_message(
    group_id: i64,
) -> Result<Message, Box<dyn std::error::Error>> {
    let mut message_str = String::from("以下为今天的水群榜：");
    let file_path = format!("./data/daily/{}.json", group_id);
    let data = read_json_file(&file_path)?;
    let mut sorted_vec: Vec<(&String, &i64)> = data.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mut idx = 0;
    for (k, v) in sorted_vec.iter() {
        if idx >= 30 {
            break;
        }
        let user_id: i64 = k.parse()?;
        if let Ok(name) = get_group_member_name(group_id, user_id).await {
            message_str = format!("{}\n{}.{}: {}", message_str, idx + 1, name, v);
            idx = idx + 1;
        }
    }
    Ok(convert_string_to_message(message_str))
}

pub async fn produce_total_report_message(
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
    let mut idx = 0;
    for (k, v) in sorted_vec.iter() {
        if idx >= 30 {
            break;
        }
        let user_id: i64 = k.parse()?;
        if let Ok(name) = get_group_member_name(group_id, user_id).await {
            message_str = format!("{}\n{}.{}: {}", message_str, idx + 1, name, v);
            idx = idx + 1;
        }
    }
    Ok(convert_string_to_message(message_str))
}

pub fn get_pet_list() -> Result<String, Box<dyn std::error::Error>> {
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
