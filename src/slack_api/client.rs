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

#[derive(Serialize, Deserialize, Debug)]
struct SlackMessageWithAttachmenets {
    channel: String,
    username: String,
    text: String,
    attachments: Vec<SlackMessageAttachments>
}

#[derive(Serialize, Deserialize, Debug)]
struct SlackMessageAttachments {
    fallback: String,
    color: String,
    pretext: String,
    author_name: String,
    author_link: String,
    author_icon: String,
    title: String,
    text: String,
    fields: SlackMessageAttachmentFields,
    image_url: String,
    thumb_url: String,
    footer: String,
    footer_icon: String,
    ts: String
}

impl Default for SlackMessageAttachments {
    fn default() -> Self {
        Self {
            fallback: String::from("fallback text"),
            color: String::from("#36a64f"),
            pretext: String::from("pretext..."),
            author_name: String::from("rust-library-sandbox"),
            author_link: String::from("https://github.com/aztecher/rust-library-sandbox"),
            author_icon: String::from(""),
            title: String::from("title..."),
            text: String::from("text..."),
            fields: Default::default(),
            image_url: String::from(""),
            thumb_url: String::from(""),
            footer: String::from(""),
            footer_icon: String::from(""),
            ts: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SlackMessageAttachmentFields {
    title: String,
    value: String,
    short: bool,
}

impl Default for SlackMessageAttachmentFields {
    fn default() -> Self {
        Self {
            title: String::from("title..."),
            value: String::from("value..."),
            short: false,
        }
    }
}

#[tokio::main]
pub async fn post_from_file(f: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    let slack: Slack = serde_json::from_reader(reader).unwrap();
    println!("{:?}", slack);
    // // simple message
    // let message = SlackMessage {
    //     channel: slack.channel,
    //     username: slack.username,
    //     text: String::from("Hello, from Rust"),
    // };
    // // rich message
    let message = SlackMessageWithAttachmenets {
        channel: slack.channel,
        username: slack.username,
        text: String::from("Hello, from Rust"),
        attachments: vec![Default::default()],
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
