extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn google_search_from_query(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    let google_search_url = format!("https://google.com/search?q={}", encoded_query);
    return google_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_search_from_query_url() {
        let query = "hello";
        assert_eq!(
            google_search_from_query(query),
            "https://google.com/search?q=hello"
        );
    }

    #[test]
    fn test_google_search_from_query_with_encoding() {
        let query = "hello world";
        assert_eq!(
            google_search_from_query(query),
            "https://google.com/search?q=hello%20world"
        );
    }
}