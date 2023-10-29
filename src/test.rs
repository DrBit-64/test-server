#[cfg(test)]
mod tests {
    use crate::io::*;
    use crate::mytype::*;
    use crate::produce::*;
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
}
