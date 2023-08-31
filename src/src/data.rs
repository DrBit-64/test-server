use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

#[derive(Serialize)]
struct Message {
    #[serde(rename = "type")]
    type_: String,
    data: HashMap<String, Value>,
}

#[derive(Serialize)]
struct PostBody {
    group_id: i64,
    message: Message,
}

fn remove_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    if input.starts_with(prefix) {
        // 使用字符串切片去除前缀
        &input[prefix.len()..]
    } else {
        input
    }
}

fn open_or_create_file(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

fn read_json_file(file_path: &str) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let mut file = open_or_create_file(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let data: HashMap<String, i64> = serde_json::from_str(&contents)?;
    Ok(data)
}

fn write_data_to_file(data: &HashMap<String, i64>, file_path: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let mut file = File::create(file_path)?;
    file.set_len(0)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
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

pub fn transfer_data() {
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

pub fn add_cnt(group_id: i64, user_id: i64) {
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
