use hyper::body::Bytes;
use serde_json::{self, Value};
use std::collections::HashMap;
pub mod data;

pub fn analyze_post_body(body: Bytes) {
    let body: HashMap<String, Value> =
        serde_json::from_slice(&body).expect("error when convert body to Hashmap");
    if let Some(post_type) = body.get("post_type") {
        if post_type == "message" && body.get("message_type").unwrap() == "group" {
            let group_id = body.get("group_id").unwrap().as_i64().unwrap();
            let user_id = body.get("user_id").unwrap().as_i64().unwrap();
            data::add_cnt(group_id, user_id);
        }
    }
}

pub fn delayed_sending() {
    todo!()
}
