#[cfg(test)]
mod tests {
    use crate::file_io::*;
    use crate::mytype::*;
    use crate::produce::*;
    use crate::src::parse_message;
    #[test]
    fn test_read_fortune() {
        println!("test_read_fortune");
        let data = read_fortune_data_from_json("./dict/fortune.json");
        println!("{:?}", data);
    }
    #[test]
    fn test_read_fortune_state() {
        println!("test_read_fortune_state");
        let data = read_fortune_state_from_json("./data/fortune/123456.json");
        println!("{:?}", data);
    }
    #[test]
    fn test_write_fortune_state() {
        println!("test_write_fortune_state");
        let data = FortuneState::new(1, vec![1, 33, 10, 2]);
        write_fortune_state_to_json("./data/fortune/114514.json", &data);
    }
    #[test]
    fn test_get_fortune_state() {
        println!("test_get_fortune_state");
        let data = get_fortune_state(114514);
        println!("{:?}", data);
        let data = get_fortune_state(12211023);
        println!("{:?}", data);
    }
    #[test]
    fn test_clear_fortune_data() {
        let res = clear_all_fortune_data();
        println!("{:?}", res);
    }
    #[test]
    fn test_produce_fortune_message() {
        let message = produce_fortune_message(114514);
        println!("{:?}", message);
        let message = produce_fortune_message(1919810);
        println!("{:?}", message);
    }
    #[test]
    fn test_parse_message() {
        let string = String::from("hello world");
        println!("{:?}", parse_message(&string));
        let string = String::from("");
        println!("{:?}", parse_message(&string));
    }
    #[test]
    fn test_clear_gpt_chat_message() {
        clear_gpt_chat_message(0);
    }
}
#[cfg(test)]
mod async_tests {
    use crate::produce::*;
    use crate::web_io::*;
    use tokio::test;
    #[test]
    async fn test_normal_chat_to_gpt() {
        let messages = String::from("");
        println!("{}", normal_chat_to_gpt(messages, 0, 0).await);
    }
    #[test]
    async fn test_load_gpt_character() {
        let file_path = "./dict/gpt-neko.json";
        load_gpt_chat_characters(file_path, 1);
        let message = String::from("你好，请让我摸摸尾巴");
        println!("{}", normal_chat_to_gpt(message, 1, 1).await);
    }
    #[test]
    async fn test_summarize_qq_message_via_gpt() {
        let message = summarize_qq_message_via_gpt(0).await;
        println!("{}", message);
    }
}
