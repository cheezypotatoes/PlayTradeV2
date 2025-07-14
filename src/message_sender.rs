
use std::{thread, vec};
use std::thread::JoinHandle;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{self, Write};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::fs::{File};
use std::collections::HashMap;
use rand::{rng, Rng};
use colored::*;
use serde_json;


use crate::ini_file_helpers;

static IS_SENDER_THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

// TODO: PRINT IF ERROR SEND
pub fn run_sender_thread(token: Vec<String>, servers: Vec<String>) -> Result<JoinHandle<()>, std::io::Error> {
    
    let is_multiple_account = ini_file_helpers::access_ini_data("post request mode", "multiple_accounts").parse::<bool>().unwrap_or(false);
    
    
    if !is_multiple_account {
        return non_multiple_account_thread(token[0].clone(), servers);
    } 
    
    multiple_account_thread(token.clone(), servers)
}

fn multiple_account_thread(tokens: Vec<String>, servers: Vec<String>) -> Result<JoinHandle<()>, std::io::Error> {
    let handle = thread::spawn(move || {
       
        let mut debounce_time_per_server = ini_file_helpers::access_ini_data("post request mode", "time_specific").parse::<u64>().unwrap_or(0);
            if debounce_time_per_server <= 0 {
                debounce_time_per_server = 5;
        }
        while IS_SENDER_THREAD_RUNNING.load(Ordering::Relaxed) {
            for token in &tokens {
                for server in &servers {
                    send_message(token, server);
                    println!("    MESSAGE SUCCESSFULLY SENT TO [{}]", format!("{}", server).truecolor(0, 128, 128));
                    
                }
            }
            thread::sleep(Duration::from_secs(debounce_time_per_server));
        }
    });

    return Ok(handle);
}


fn non_multiple_account_thread(token: String, servers: Vec<String>) -> Result<JoinHandle<()>, std::io::Error> {
    let handle = thread::spawn(move || {
        let mut debounce_time_per_server = ini_file_helpers::access_ini_data("post request mode", "time_specific").parse::<u64>().unwrap_or(0);
        if debounce_time_per_server <= 0 {
            debounce_time_per_server = 5;
        }

        while IS_SENDER_THREAD_RUNNING.load(Ordering::Relaxed) {
            for server in &servers {
                send_message(&token, server);
                println!("    MESSAGE SUCCESSFULLY SENT TO [{}]", format!("{}", server).truecolor(0, 128, 128));
            }
            thread::sleep(Duration::from_secs(debounce_time_per_server));
        }
    });

    return Ok(handle);
}


fn get_all_tokens() -> Vec<String> {
    let multiple_accounts = ini_file_helpers::access_ini_data("post request mode", "multiple_accounts").parse::<bool>().unwrap_or(false);
    let file_read = File::open("tokens.json").unwrap();
    let primary_in_ini = ini_file_helpers::access_ini_data("settings", "primary");
    let token_map: HashMap<String, String> = serde_json::from_reader(file_read).unwrap();

    if multiple_accounts {
        return token_map.values().cloned().collect();
    } else {

        if token_map.contains_key(&primary_in_ini) {
            let token = token_map[primary_in_ini.trim()].clone();
            return vec![token];

        } else {

            if let Some((_key, value)) = token_map.iter().next() {
                return vec![value.to_string()];
            } else {
                return Vec::new();
            }

        }
    }


}


fn get_all_servers() -> Vec<String> {
    let file_read = File::open("server.json").unwrap();
    let server_map: HashMap<String, String> = serde_json::from_reader(file_read).unwrap();

    return server_map.values().cloned().collect();
}

fn send_message(authorization_token: &str, server_url: &str){
    
    let mut message = ini_file_helpers::access_ini_data("settings", "message");
    let is_font_randomized= ini_file_helpers::access_ini_data("message mode", "randomize_fonts").parse::<bool>().unwrap_or(false);

    if is_font_randomized {
        message = font_randomizer(&message);
    }
       

    let json_body = format!(r#"{{ "content": "{}" }}"#, message);

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(authorization_token).expect("Invalid header value"),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = Client::new();
    let response = client
        .post(server_url)
        .headers(headers)
        .body(json_body)
        .send();

    match response {
        Ok(resp) => {
            match resp.text() {
                Ok(_body) => {},
                Err(e) => {
                    eprintln!("Failed to read response body: {}", e);
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to send message: {}", e);
        }
    }
}

pub fn message_sender_main() {
    let mut player_option_picked = String::new(); 
    let tokens = get_all_tokens();
    let servers = get_all_servers();

    if tokens.is_empty() {
        println!("    Token Not Found");
        return;
    }

    toggle_thread_boolean();

    let handle = match run_sender_thread(tokens, servers) {
        Ok(handle) => handle,
        Err(e) => {
            println!("    ERROR [{}]", e);
            return;
        }
    };

    println!("    -- {} --", "SENDING MESSAGE (TYPE ANY KEYS TO STOP)".truecolor(0, 128, 128).bold());

    player_option_picked.clear();
    io::stdout().flush().unwrap();

    io::stdin()
    .read_line(&mut player_option_picked)
    .expect("Failed to read line");

    // Any key typed will stop the thread
    if !player_option_picked.is_empty() {
        toggle_thread_boolean();
        handle.join().unwrap();
        return;
    }

        
}

fn font_randomizer(message: &str) -> String {
    let fonts = ["**", "||", "***", "*", ""];
    let mut rng = rng();
    let num = rng.random_range(0..=3);
    

    return format!("{}{}{}", fonts[num], message, fonts[num]);
}


fn toggle_thread_boolean() {
    IS_SENDER_THREAD_RUNNING.store(!IS_SENDER_THREAD_RUNNING.load(Ordering::Relaxed), Ordering::Relaxed)
}


