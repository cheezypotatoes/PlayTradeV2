use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

fn main() {
    let url = "https://discord.com/api/v9/channels/1074719811264851998/messages";

    let json_body = r#"{
        "content": "Hello!, Goodbye"
    }"#;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("MTE3OTkyMzMzOTUwNzE0MjY4Ng.Gga6ay.0NLYGgLL_14VOFArxQqtD5U5Bz8GEM2XW1s8M8"),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .body(json_body)
        .send();

    match response {
        Ok(_resp) => {
            print!("Successfully sent")
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }
}

fn change_message(message: &String) -> String{
    format!(r#"{{ "content": "{}" }}"#, message)
}

fn send_message(url: &str, token: &str, message: &str) {
    
}