use crate::io::*;
use crate::mytype::*;
use base64::encode;
use hyper::{body::HttpBody, Body, Client, Method, Request};
use rand::prelude::SliceRandom;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn convert_string_to_message(s: String) -> Message {
    let mut data: HashMap<String, Value> = HashMap::new();
    data.insert(String::from("text"), Value::String(s));
    Message::new(String::from("text"), data)
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

pub async fn get_avatar(user_id: i64) {
    let avatar_path = format!("data/avatar/{}.jpg", user_id);
    let due_path = format!("data/avatar/{}.due", user_id);
    //judge if the file at avatar_path is exist
    if let Ok(_) = fs::metadata(&avatar_path) {
        //judge if the file at due_path is exist
        if let Ok(_) = fs::metadata(&due_path) {
            //judge if the file at due_path is due
            let due_time = fs::metadata(&due_path).unwrap().modified().unwrap();
            let now = std::time::SystemTime::now();
            let now = now.duration_since(std::time::UNIX_EPOCH).unwrap();
            let now = now.as_secs();
            let due_time = due_time.duration_since(std::time::UNIX_EPOCH).unwrap();
            let due_time = due_time.as_secs();
            if now - due_time < 86400 {
                return;
            }
        }
    }
    //create the file at avatar_path
    //use hyper to get the avatar
    let mut file = fs::File::create(&avatar_path).unwrap();
    let url = format!("http://q1.qlogo.cn/g?b=qq&nk={}&s=640", user_id);
    let url = url.parse::<hyper::Uri>().unwrap();
    let client = Client::new();
    let mut response = client.get(url).await.unwrap();
    let mut body = Vec::new();
    while let Some(chunk) = response.data().await {
        let chunk = chunk.unwrap();
        body.extend_from_slice(&chunk);
    }
    file.write_all(&body).unwrap();
    //create the file at due_path
    let mut file = fs::File::create(&due_path).unwrap();
    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap();
    let now = now.as_secs();
    let now = now.to_string();
    file.write_all(now.as_bytes()).unwrap();
}

pub fn get_wife(group_id: i64, user_id: i64) -> Result<i64, Box<dyn std::error::Error>> {
    let target_path = format!("./data/wife/{}.json", group_id);
    let wife_connection = read_json_file(&target_path)?;
    if let Some(wife) = wife_connection.get(&user_id.to_string()) {
        return Ok(wife.to_owned());
    }
    let file_path = format!("./data/total/{}.json", group_id);
    let active_members: Vec<i64> = read_json_file(&file_path)?
        .iter()
        .filter(|(_, v)| **v > 30)
        .map(|(x, _)| x.parse::<i64>().unwrap())
        .filter(|x| *x != user_id && !wife_connection.values().any(|v| v == x))
        .collect();
    let mut rng = rand::thread_rng();
    let wife = active_members.choose(&mut rng).unwrap().to_owned();
    let mut wife_connection = read_json_file(&target_path)?;
    wife_connection.insert(user_id.to_string(), wife);
    write_data_to_file(&wife_connection, &target_path)?;
    Ok(wife)
}

pub async fn get_wife_message(
    group_id: i64,
    user_id: i64,
) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let mut messages: Vec<Message> = Vec::new();
    let wife = get_wife(group_id, user_id)?;
    let at_message = Message::new(String::from("at"), {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(String::from("qq"), Value::String(user_id.to_string()));
        data
    });
    messages.push(at_message);
    let text_message = Message::new(String::from("text"), {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(
            String::from("text"),
            Value::String(String::from("你的今日老婆是")),
        );
        data
    });
    messages.push(text_message);
    let _ = get_avatar(wife).await;
    let avatar_path = format!("data/avatar/{}.jpg", wife);
    let image_bytes = fs::read(avatar_path).expect("Failed to read image file");
    let base64_encoded = encode(&image_bytes);
    let image_message = Message::new(String::from("image"), {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(
            String::from("file"),
            Value::String(format!("base64://{}", base64_encoded)),
        );
        data
    });
    messages.push(image_message);
    let wife_name = get_group_member_name(group_id, wife).await.unwrap();
    let text_message = Message::new(String::from("text"), {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(
            String::from("text"),
            Value::String(format!("{}({})", wife_name, wife)),
        );
        data
    });
    messages.push(text_message);
    Ok(messages)
}
