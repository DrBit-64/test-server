use crate::file_io::*;
use crate::produce::*;
use crate::web_io::*;
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

fn parse_message(message: &str) -> (&str, Vec<&str>) {
    let mut iter = message.split_whitespace();
    let command = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();
    (command, args)
}

async fn analyze_message(body: HashMap<String, Value>) {
    let group_id = body.get("group_id").unwrap().as_i64().unwrap();
    let user_id = body.get("user_id").unwrap().as_i64().unwrap();
    let raw_message = body.get("raw_message").unwrap().as_str().unwrap();
    if !raw_message.starts_with("!!") {
        add_cnt(group_id, user_id);
        return;
    }
    let (command, args) = parse_message(raw_message);
    match command {
        "!!ping" => send_string_to_group(String::from("pong!!"), group_id)
            .await
            .unwrap(),
        "!!daily-rank" => {
            let message = produce_daily_report_message(group_id).await.unwrap();
            send_message_to_group(message, group_id).await.unwrap()
        }
        "!!total-rank" => {
            let message = produce_total_report_message(group_id).await.unwrap();
            send_message_to_group(message, group_id).await.unwrap();
        }
        "!!petpet-list" => {
            let pet_list = get_pet_list().unwrap();
            send_string_to_group(pet_list, group_id).await.unwrap();
        }
        "!!群友老婆" => {
            let messages = get_wife_message(group_id, user_id).await.unwrap();
            send_messages_to_group(messages, group_id).await.unwrap();
        }
        "!!抽签" => {
            let messages = produce_fortune_message(user_id);
            send_messages_to_group(messages, group_id).await.unwrap();
        }
        "!!chat" => {
            let input_messages = args.join(" ");
            let gpt_request_body: crate::mytype::GPTRequestBody =
                transfer_single_message_to_gpt_request_body(input_messages);
            let response_string = send_message_to_gpt(gpt_request_body).await;
            send_string_to_group(response_string, group_id)
                .await
                .unwrap();
        }
        _ => add_cnt(group_id, user_id),
    }
}

pub async fn analyze_post_body(body: Bytes) {
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("analyze post body finished");
    let body: HashMap<String, Value> =
        serde_json::from_slice(&body).expect("error when convert body to Hashmap");
    if let Some(post_type) = body.get("post_type") {
        if post_type == "message" && body.get("message_type").unwrap() == "group" {
            analyze_message(body).await;
        }
    }
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
    clear_all_wife_data()?;
    clear_all_fortune_data()?;
    Ok(())
}
