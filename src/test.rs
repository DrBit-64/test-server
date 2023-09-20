#[cfg(test)]
mod test {
    use crate::io::*;
    use crate::produce::*;
    #[tokio::test]
    async fn test_get_avatar() {
        let _ = get_avatar(328808246).await;
        let _ = get_avatar(2148431973).await;
    }
    #[test]
    fn test_get_wife() {
        let _ = get_wife(863770345, 328808246);
        let _ = get_wife(863770345, 1489952006);
        let _ = get_wife(863770345, 2148431973);
    }
    #[test]
    fn test_clear_wife_data() {
        let _ = clear_all_wife_data();
    }
}
