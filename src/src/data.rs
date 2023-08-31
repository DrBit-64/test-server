use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, Read};

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

pub fn add_cnt(group_id: i64, user_id: i64) {
    let file_path = "./data/".to_owned() + &group_id.to_string() + "_daily.json";
    match read_json_file(&file_path) {
        Ok(mut data) => {
            let counter = data.entry(user_id.to_string()).or_insert(0);
            *counter += 1;
            match write_data_to_file(&data, &file_path) {
                Err(err) => println!("error when write json to daily file: {:?}", err),
                _ => (),
            }
        }
        Err(err) => println!("error when read json from daily file {:?}", err),
    }
}
