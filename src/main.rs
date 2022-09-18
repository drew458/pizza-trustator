use std::{collections::HashMap, time::SystemTime};
use arboard::Clipboard;

use reqwest::{Client, Response};
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

    let google_response = google_translation(&client, &request).await?;

    match google_response.status() {
        reqwest::StatusCode::OK => {
            match google_response.json::<ResponseReturned>().await {
                Ok(response_parsed) => println!("Result: {}\n", response_parsed.translation),
                Err(_) => print!("Unexpected response returned.")
            };
        },
        _ => {
            panic!("Unexpected error.");
        },
    };

    println!("Translated in {:?} milliseconds", SystemTime::now().duration_since(start).unwrap().as_millis());
    Ok(())
}

async fn google_translation(client: &Client, request: &HashMap<&str, &str>) -> Result<Response, reqwest::Error> {
    return client.post("https://deep-translator-api.azurewebsites.net/google/")
        .header(reqwest::header::ACCEPT, "application/json")
        .json(request)
        .send().await;
}