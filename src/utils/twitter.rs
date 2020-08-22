extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn twitter_url(query: &str) -> String {
    if query == "tw"{
        return String::from("https://twitter.com");
    } else if &query[..4] == "tw @"{
        return twitter_profile_url(&query[4..]);
    } else{
        return twitter_search_url(&query[3..]);
    }
}

pub fn twitter_profile_url(profile: &str) -> String {
    return format!("https://twitter.com/{}", profile)
}

pub fn twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT);
    let twitter_search_url = format!("https://twitter.com/search?q={}", encoded_query);
    return twitter_search_url
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_twitter_url() {
        let query = "tw";
        assert_eq!(
            twitter_url(query),
            "https://twitter.com"
        );
    }

    #[test]
    fn test_twitter_url_with_query() {
        let query = "tw hello world";
        assert_eq!(
            twitter_url(query),
            "https://twitter.com/search?q=hello%20world"
        );
    }

    #[test]
    fn test_twitter_profile_url() {
        let profile = "tw @TheTweetOfSKR";
        assert_eq!(
            twitter_url(profile),
            "https://twitter.com/TheTweetOfSKR"
        );
    }

    #[test]
    fn test_twitter_profile_url_alt() {
        let profile = "TheTweetOfSKR";
        assert_eq!(
            twitter_profile_url(profile),
            "https://twitter.com/TheTweetOfSKR"
        );
    }

    #[test]
    fn test_twitter_search_url() {
        let query = "hello world";
        assert_eq!(
            twitter_search_url(query),
            "https://twitter.com/search?q=hello%20world"
        );
    }
}