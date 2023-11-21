use tokio::join;
use tokio::time::{sleep, Duration};

use crate::file_io::{
    clear_dailogue_data, read_dialogue_data_from_json, write_dialogue_data_to_json,
};
use crate::mytype::{ChatMessage, GPTModel, GPTRequestBody};
use crate::produce::get_group_member_name;

pub fn transfer_single_string_to_gpt_request_body(message: String) -> GPTRequestBody {
    let gpt_message = ChatMessage::default(message);
    let messages = vec![gpt_message];
    let gpt_model = GPTModel::default();
    GPTRequestBody::new(gpt_model, messages)
}

pub fn transfer_messages_to_gpt_request_body(messages: Vec<ChatMessage>) -> GPTRequestBody {
    let gpt_model = GPTModel::default();
    GPTRequestBody::new(gpt_model, messages)
}

fn storage_chat_message(file_path: &str, new_message: ChatMessage, max_length_limit: usize) {
    let mut data = read_dialogue_data_from_json(file_path);
    data.push(new_message);
    while data
        .iter()
        .map(|x| x.content.chars().count())
        .sum::<usize>()
        > max_length_limit
        || data.iter().count() > 200
    {
        data.remove(0);
    }
    write_dialogue_data_to_json(file_path, &data);
}

fn clear_chat_message(file_path: &str) {
    clear_dailogue_data(file_path);
}

pub fn transfer_chat_message_to_string(data: &Vec<ChatMessage>, cnt: usize) -> String {
    let mut result = String::new();
    let len = data.len();
    let last_n_element = if len > cnt { cnt } else { len };
    let data = &data[len - last_n_element..len];
    for message in data {
        result = format!("{}{}:{}\n", result, message.role, message.content);
    }
    result
}

pub async fn storage_qq_message_to_file(message: String, group_id: i64, user_id: i64) {
    let file_path = format!("./data/dialogue/qq/{}.json", group_id);
    if message.is_empty() {
        return;
    }
    let role = get_group_member_name(group_id, user_id).await.unwrap();
    let new_message = ChatMessage::new(role, message);
    storage_chat_message(&file_path, new_message, 3000);
}

fn storage_gpt_message_to_file(message: ChatMessage, user_id: i64) {
    let file_path = format!("./data/dialogue/gpt/{}.json", user_id);
    storage_chat_message(&file_path, message, 3000);
}

fn get_qq_message_from_file(group_id: i64) -> Vec<ChatMessage> {
    let file_path = format!("./data/dialogue/qq/{}.json", group_id);
    read_dialogue_data_from_json(&file_path)
}

fn get_gpt_message_from_file(user_id: i64) -> Vec<ChatMessage> {
    let file_path = format!("./data/dialogue/gpt/{}.json", user_id);
    read_dialogue_data_from_json(&file_path)
}

pub fn clear_gpt_chat_message(user_id: i64) {
    let file_path = format!("./data/dialogue/gpt/{}.json", user_id);
    clear_chat_message(&file_path);
}

pub async fn normal_chat_to_gpt(messages: String, user_id: i64) -> String {
    let new_message = ChatMessage::default(messages.clone());
    let history_message = get_gpt_message_from_file(user_id);
    let mut messages = history_message;
    messages.push(new_message.clone());
    let request_body = transfer_messages_to_gpt_request_body(messages);
    storage_gpt_message_to_file(new_message, user_id);
    let response_string = crate::web_io::send_message_to_gpt(request_body);
    let sleep_handler = sleep(Duration::from_secs(2));
    let (response_string, _) = join!(response_string, sleep_handler);
    storage_gpt_message_to_file(
        ChatMessage::new(String::from("assistant"), response_string.clone()),
        user_id,
    );
    response_string
}
pub fn load_gpt_chat_characters(source_file_path: &str, user_id: i64) {
    let source_messages = read_dialogue_data_from_json(source_file_path);
    for message in source_messages {
        storage_gpt_message_to_file(message, user_id);
    }
}

pub async fn summarize_qq_message_via_gpt(group_id: i64, cnt: usize) -> String {
    let mut request_string =
        String::from("以下为一段群聊的聊天记录，请总结主要聊天内容并概括每个人的聊天方式与性格\n");
    let qq_chat_messages = get_qq_message_from_file(group_id);
    request_string = format!(
        "{}{}",
        request_string,
        transfer_chat_message_to_string(&qq_chat_messages, cnt)
    );
    let request_body = transfer_single_string_to_gpt_request_body(request_string);
    crate::web_io::send_message_to_gpt(request_body).await
}
