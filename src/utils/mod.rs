pub mod google;
pub mod twitter;

pub fn get_cmd_from_query(query_str: &str) -> &str {
    if query_str.contains(' '){
        let index_white_space = query_str.find(' ').unwrap_or(0);
        return &query_str[..index_white_space];
    }

    return &query_str
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_get_cmd_from_query_no_whitespace() {
        let actual = get_cmd_from_query("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_cmd_from_query_with_whitespace() {
        let actual = get_cmd_from_query("tw @TheTweetOfSKR");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}


