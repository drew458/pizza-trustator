use std::{collections::HashMap, time::SystemTime};
use arboard::Clipboard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ResponseReturned {
    translation: String,
    error: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let start = SystemTime::now();

    let mut clipboard = Clipboard::new().unwrap();
    let clipboard_text = clipboard.get_text().unwrap();

    println!("\nTranslating: {}", clipboard_text);

    let mut request = HashMap::new();
    request.insert("source", "auto");
    request.insert("target", "italian");
    request.insert("text", &clipboard_text);

    let client = reqwest::Client::new();

    let response = client.post("https://deep-translator-api.azurewebsites.net/google/")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ResponseReturned>().await {
                Ok(response_parsed) => println!("Result: {}\n", response_parsed.translation),
                Err(_) => print!("Unexpected response returned.")
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        },
        _ => {
            panic!("Unexpected error.");
        },
    };

    println!("Translated in {:?} milliseconds", SystemTime::now().duration_since(start).unwrap().as_millis());
    Ok(())
}
