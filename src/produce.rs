use crate::file_io::*;
use crate::mytype::*;
use base64::encode;
pub use dialogue::*;
use hyper::{body::HttpBody, Client};
use rand::prelude::SliceRandom;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
pub use utils::*;
mod dialogue;
mod utils;

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
        .filter(|(_, v)| **v > 200)
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

pub fn get_fortune_state(user_id: i64) -> FortuneState {
    let file_path = format!("./data/fortune/{}.json", user_id);
    match read_fortune_state_from_json(&file_path) {
        Ok(state) => state,
        Err(_) => {
            let level_choices = [1, 2, 3, 4, 5];
            let weights = [1, 2, 3, 2, 1];
            let mut rng = rand::thread_rng();
            let level = level_choices
                .choose_weighted(&mut rng, |item| weights[*item - 1])
                .unwrap();

            let fortune_data = read_fortune_data_from_json("./dict/fortune.json");
            let len = fortune_data.len();
            let mut rng = rand::thread_rng();
            let mut index: Vec<_> = (0..len).collect();
            index.shuffle(&mut rng);
            index.truncate(if *level == 1 || *level == 5 { 2 } else { 4 });
            let fortune_state = FortuneState::new(*level, index);
            write_fortune_state_to_json(&file_path, &fortune_state);
            fortune_state
        }
    }
}

pub fn discode_fortune_state(state: FortuneState) -> Message {
    let file_path = "./dict/fortune.json";
    let fortune_data = read_fortune_data_from_json(file_path);
    let mut message: String;
    message = format!(
        "你的今日运势为\n§{}§",
        match state.level {
            1 => "大吉",
            2 => "中吉",
            3 => "中平",
            4 => "中凶",
            _ => "大凶",
        }
    );
    match state.level {
        1 => {
            for i in state.index {
                let data = fortune_data.get(i).unwrap();
                message = format!("{}\n宜：{} ({})", message, data.text, data.result1)
            }
        }
        5 => {
            for i in state.index {
                let data = fortune_data.get(i).unwrap();
                message = format!("{}\n忌：{} ({})", message, data.text, data.result2);
            }
        }
        _ => {
            for (idx, i) in state.index.into_iter().enumerate() {
                if idx <= 1 {
                    let data = fortune_data.get(i).unwrap();
                    message = format!("{}\n宜：{} ({})", message, data.text, data.result1)
                } else {
                    let data = fortune_data.get(i).unwrap();
                    message = format!("{}\n忌：{} ({})", message, data.text, data.result2)
                }
            }
        }
    }
    convert_string_to_message(message)
}

pub fn produce_fortune_message(user_id: i64) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();
    let at_message = Message::new(String::from("at"), {
        let mut data: HashMap<String, Value> = HashMap::new();
        data.insert(String::from("qq"), Value::String(user_id.to_string()));
        data
    });
    messages.push(at_message);
    let fortune_state = get_fortune_state(user_id);
    let text_message = discode_fortune_state(fortune_state);
    messages.push(text_message);
    messages
}
