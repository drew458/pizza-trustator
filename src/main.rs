use std::collections::HashMap;
use arboard::Clipboard;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseReturned {
    translation: String,
    error: String
}

#[tokio::main]
async fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let clipboard_text = clipboard.get_text().unwrap();

    let mut map = HashMap::new();
    map.insert("source", "auto");
    map.insert("target", "italian");
    map.insert("text", &clipboard_text);

    let client = reqwest::Client::new();

    let res = client.post("https://deep-translator-api.azurewebsites.net/google/")
        .json(&map)
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<ResponseReturned>().await {
                Ok(parsed) => println!("{:?}", parsed.translation),
                Err(_) => print!("Unexpected response returned.")
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        },
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        },
    };
}
