use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Slack {
    url: String,
    channel: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SlackMessage {
    channel: String,
    username: String,
    text: String,
}

#[tokio::main]
pub async fn post_from_file(f: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let slack: Slack = serde_json::from_reader(reader).unwrap();
    println!("{:?}", slack);
    let message = SlackMessage {
        channel: slack.channel,
        username: slack.username,
        text: String::from("Hello, from Rust"),
    };
    println!("{:?}", message);
    let client = reqwest::Client::new();
    let response = client.post(slack.url)
        .json(&message)
        .send()
        .await?;
    println!("response = {:?}", response);
    Ok(())
}
