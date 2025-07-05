use std::collections::HashSet;
use std::io::{self, Write};

// TODO: Message settings with primary
// TODO: Mode 
// TODO: Start

mod bootstrap;
mod token_settings;
mod server_settings;
mod ini_file_helpers;
mod message_settings;

fn main() {
    bootstrap::bootstrap_main();
    let allowed_text: HashSet<&str> = HashSet::from(["1", "2", "3", "4", "5", "6"]);
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
        "1" => {println!("start")},
        "2" => {token_settings::token_setting_main()},
        "3" => {server_settings::server_setting_main()},
        "4" => {message_settings::message_setting_main()},
        "5" => {println!("mode")},
        _ => {}
    }
    }

   

    
}

fn main_menu_show() {
    println!(r#"
    ____  _               _____              _      
    |  _ \| | __ _ _   _  |_   _| __ __ _  __| | ___ 
    | |_) | |/ _` | | | |   | || '__/ _` |/ _` |/ _ \
    |  __/| | (_| | |_| |   | || | (_| | (_| |  __/
    |_|   |_|\__,_|\__, |   |_||_|  \__,_|\__,_|\___|
                    |___/                        V2
    "#);

    println!(r#"
    [1]. Start Message.
    [2]. Token Settings.
    [3]. Server Settings.
    [4]. Message Settings.
    [5]. Message Mode.
    [6]. Exit.
    "#)
}

