use std::collections::HashMap;

use reqwest::header;

pub fn send_slack_message_to_channel(channel: &str, message: &str) {
    let mut map = HashMap::new();
    map.insert("channel", channel);
    map.insert("text", message);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("Bearer SLACK_TOKEN_HERE"),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client.post("https://slack.com/api/chat.postMessage").json(&map).send();
}
