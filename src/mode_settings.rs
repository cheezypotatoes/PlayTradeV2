use std::collections::HashSet;
use std::io::{self, Write};


use crate::ini_file_helpers;

pub fn mode_setting_main() {
    let allowed_text: HashSet<&str> = HashSet::from(["0","1", "2", "3", "4", "5", "6"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "6" {
        
        mode_setting_menu_show();
        print!("    PlayTrade <Mode Setting>: ");
        player_option_picked.clear();
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut player_option_picked)
        .expect("Failed to read line");

        if !allowed_text.contains(player_option_picked.trim()) {
            println!("    Option not found");
        }

        match player_option_picked.trim() {
            "0" => {  change_boolean_ini_data("Post Request Mode", "multiple_servers"); },
            "1" => {  change_boolean_ini_data("Post Request Mode", "multiple_accounts"); },
            "2" => {  change_number_ini_data("Post Request Mode", "time_randomizer"); },
            "3" => {  change_number_ini_data("Post Request Mode", "time_specific"); },
            "4" => {  change_boolean_ini_data("Message Mode", "randomize_fonts"); },
            "5" => {  change_boolean_ini_data("Message Mode", "emoji_injection"); },
            _ => {},
        }
    }   
}


fn mode_setting_menu_show() {
    let time_randomizer = ini_file_helpers::access_ini_data("Post Request Mode", "time_randomizer");
    let time_specific = ini_file_helpers::access_ini_data("Post Request Mode", "time_specific");
    println!("    -- Post Request Modes --");
    println!("    [0]:Multiple Servers  [{}]", ini_file_helpers::access_ini_data("Post Request Mode", "multiple_servers").to_uppercase());
    println!("    [1]:Multiple Accounts [{}]", ini_file_helpers::access_ini_data("Post Request Mode", "multiple_accounts").to_uppercase());
    println!("    [2]:Time Randomizer   [{}]{}", (time_randomizer != "0").to_string().to_uppercase(), if time_randomizer != "0" { format!(" [{} Minutes]", time_randomizer) } else { String::new() });
    println!("    [3]:Time Specific     [{}]{}", (time_specific != "0").to_string().to_uppercase(), if time_specific != "0" { format!(" [{} Minutes]", time_specific) } else { String::new() });

    println!("\n");

    println!("    -- Message Modes --");
    println!("    [4]:Randomize Fonts   [{}]", ini_file_helpers::access_ini_data("Message Mode", "randomize_fonts").to_uppercase());
    println!("    [5]:Emoji Injection   [{}]", ini_file_helpers::access_ini_data("Message Mode", "emoji_injection").to_uppercase()); 

    println!("\n    Type The Number To Toggle.");

    println!(
    "
    [6]. Return.
    "
    );
}

fn change_boolean_ini_data(category: &str, key: &str) {
    let value_str = ini_file_helpers::access_ini_data(category, key);
    let value: bool = !value_str.parse::<bool>().unwrap_or(false);
    ini_file_helpers::edit_ini_data(category, &value.to_string(), key);
}

fn change_number_ini_data(category: &str, key: &str) {
    let mut time = String::new();
    print!("    Maximum Minutes: ");
    io::stdout().flush().unwrap();
    let result = io::stdin().read_line(&mut time);
    match result {
        Ok(_num) => {},
        Err(_) => {
            println!("    Invalid Input entered.");
            return;
        }
    };

    match time.trim().parse::<i32>() {
        Ok(_num) => {},
        Err(_) => {
            println!("    Invalid number entered.");
            return;
        }
    }

    ini_file_helpers::edit_ini_data(category, "0", "time_randomizer");
    ini_file_helpers::edit_ini_data(category, "0", "time_specific");

    ini_file_helpers::edit_ini_data(category,  &time, key);
}


