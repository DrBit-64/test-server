use crate::mytype::*;
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::Read;

pub fn remove_prefix<'a>(input: &'a str, prefix: &str) -> &'a str {
    if input.starts_with(prefix) {
        // 使用字符串切片去除前缀
        &input[prefix.len()..]
    } else {
        input
    }
}

pub fn open_or_create_file(file_path: &str) -> Result<File, Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    Ok(file)
}

pub fn read_json_file(file_path: &str) -> Result<HashMap<String, i64>, Box<dyn std::error::Error>> {
    let mut file = open_or_create_file(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        return Ok(HashMap::new());
    }
    let data: HashMap<String, i64> = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn write_data_to_file(data: &HashMap<String, i64>, file_path: &str) -> std::io::Result<()> {
    let json_data = serde_json::to_string(data)?;
    let mut file = File::create(file_path)?;
    file.set_len(0)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn read_fortune_data_from_json(file_path: &str) -> Vec<FortuneData> {
    let mut file = open_or_create_file(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: Vec<FortuneData> = serde_json::from_str(&contents).unwrap();
    data
}

pub fn read_fortune_state_from_json(file_path: &str) -> Result<FortuneState, Box<dyn Error>> {
    let mut file = open_or_create_file(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: FortuneState = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn write_fortune_state_to_json(file_path: &str, state: &FortuneState) {
    let mut file = open_or_create_file(file_path).unwrap();
    let data = state;
    let json_data = serde_json::to_string(&data).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();
}

pub fn clear_all_wife_data() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir("./data/wife")?;
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let file_path = format!("./data/wife/{}", file_name);
                let mut data = read_json_file(&file_path)?;
                data.clear();
                write_data_to_file(&data, &file_path)?;
            }
        }
    }
    Ok(())
}

pub fn clear_all_fortune_data() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./data/fortune";
    let entries = fs::read_dir(file_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path)?;
        } else if path.is_dir() {
            fs::remove_dir_all(path)?;
        }
    }
    Ok(())
}
