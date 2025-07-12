use std::collections::HashSet;
use std::io::{self, Write};
use colored::*;



// TODO: Start

mod bootstrap;
mod token_settings;
mod server_settings;
mod ini_file_helpers;
mod message_settings;
mod mode_settings;
mod message_sender;

fn main() {
    bootstrap::bootstrap_main();
    let allowed_text: HashSet<&str> = HashSet::from(["1", "2", "3", "4", "5", "6", "7"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "6" {
        
        main_menu_show();
        print!("    PlayTrade <Main Menu>: ");
        player_option_picked.clear();
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut player_option_picked)
        .expect("Failed to read line");

        if !allowed_text.contains(player_option_picked.trim()) {
            println!("    Option not found");
        }

        match player_option_picked.trim() {
        "1" => {message_sender::message_sender_main()},
        "2" => {token_settings::token_setting_main()},
        "3" => {server_settings::server_setting_main()},
        "4" => {message_settings::message_setting_main()},
        "5" => {mode_settings::mode_setting_main()},
        _ => {}
    }
    }

   

    
}

fn main_menu_show() {
    println!(
        "{}",
        r#"
    ____  _               _____              _      
    |  _ \| | __ _ _   _  |_   _| __ __ _  __| | ___ 
    | |_) | |/ _` | | | |   | || '__/ _` |/ _` |/ _ \
    |  __/| | (_| | |_| |   | || | (_| | (_| |  __/
    |_|   |_|\__,_|\__, |   |_||_|  \__,_|\__,_|\___|
                    |___/                        V2
    "#
        .bright_cyan()
        .bold()
    );

    let teal = |text: &str| text.truecolor(0, 128, 128);
    let menu = [
        "Start Message.",
        "Token Settings.",
        "Server Settings.",
        "Message Settings.",
        "Message Mode.",
        "Exit.",
    ];

    for (i, item) in menu.iter().enumerate() {
        println!("    {} {}", teal(&format!("[{}]", i + 1)), item);
    }
}

