use std::collections::HashSet;
use std::io::{self, Write};
use colored::*;

use crate::ini_file_helpers;

pub fn mode_setting_main() {
    let allowed_text: HashSet<&str> = HashSet::from(["0","1", "2", "3", "4", "5", "6"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "5" {
        
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
            "0" => {  change_boolean_ini_data("Post Request Mode", "multiple_accounts"); },
            "1" => {  change_number_ini_data("Post Request Mode", "time_randomizer"); },
            "2" => {  change_number_ini_data("Post Request Mode", "time_specific"); },
            "3" => {  change_boolean_ini_data("Message Mode", "randomize_fonts"); },
            "4" => {  change_boolean_ini_data("Message Mode", "emoji_injection"); },
            _ => {},
        }
    }   
}


fn colorize_bool(value: &str) -> colored::ColoredString {
    let val = value.to_uppercase();
    if val == "TRUE" {
        val.truecolor(0, 200, 0)
    } else {
        val.truecolor(200, 0, 0)
    }
    .bold()
}

fn mode_setting_menu_show() {
    let time_randomizer = ini_file_helpers::access_ini_data("Post Request Mode", "time_randomizer");
    let time_specific = ini_file_helpers::access_ini_data("Post Request Mode", "time_specific");
    println!("\n    {}", "-- Post Request Modes --".truecolor(0, 128, 128).bold());
    println!(
        "    {}:Multiple Accounts [{}]",
        "[0]".truecolor(0, 128, 128).bold(),
        colorize_bool(&ini_file_helpers::access_ini_data("Post Request Mode", "multiple_accounts"))
    );
    let time_randomizer_bool = (time_randomizer != "0").to_string();
    println!(
        "    {}:Time Randomizer   [{}]{}",
        "[1]".truecolor(0, 128, 128).bold(),
        colorize_bool(&time_randomizer_bool),
        if time_randomizer != "0" {
            format!(" [{} Minutes]", time_randomizer)
        } else {
            String::new()
        }
    );
    let time_specific_bool = (time_specific != "0").to_string();
    println!(
        "    {}:Time Specific     [{}]{}",
        "[2]".truecolor(0, 128, 128).bold(),
        colorize_bool(&time_specific_bool),
        if time_specific != "0" {
            format!(" [{} Minutes]", time_specific)
        } else {
            String::new()
        }
    );

    println!("\n");

    println!("    {}", "-- Message Modes --".truecolor(0, 128, 128).bold());
    println!(
        "    {}:Randomize Fonts   [{}]",
        "[3]".truecolor(0, 128, 128).bold(),
        colorize_bool(&ini_file_helpers::access_ini_data("Message Mode", "randomize_fonts"))
    );
    println!(
        "    {}:Emoji Injection   [{}]",
        "[4]".truecolor(0, 128, 128).bold(),
        colorize_bool(&ini_file_helpers::access_ini_data("Message Mode", "emoji_injection"))
    );

    println!("\n    Type The Number To Toggle.");

    println!(
        "    {}. Return.\n",
        "[5]".truecolor(0, 128, 128).bold()
    );
}

fn change_boolean_ini_data(category: &str, key: &str) {
    let value_str = ini_file_helpers::access_ini_data(category, key);
    let value: bool = !value_str.parse::<bool>().unwrap_or(false);
    ini_file_helpers::edit_ini_data(category, &value.to_string(), key);
}

fn change_number_ini_data(category: &str, key: &str) {
    let mut time = String::new();
    print!("    Maximum Second: ");
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


