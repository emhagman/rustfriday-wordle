use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use reqwest::header;
use std::collections::HashMap;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn send_slack_message_to_channel(channel: &str, message: &str) {
    let mut map = HashMap::new();
    map.insert("channel", channel);
    map.insert("text", message);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static("Bearer xoxb-11396693347-3716642973474-SJeinCCLQXAX8wV5z1alPh9X"),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client.post("https://slack.com/api/chat.postMessage").json(&map).send();
}
