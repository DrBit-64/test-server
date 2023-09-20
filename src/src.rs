use crate::produce::*;
use crate::utils::*;
use hyper::body::Bytes;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

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
