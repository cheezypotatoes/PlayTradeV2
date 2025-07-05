use std::collections::HashSet;
use std::io::{self, Write};

use crate::ini_file_helpers;

pub fn message_setting_main() {
    let allowed_text: HashSet<&str> = HashSet::from(["1", "2"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "4" {
        
        message_setting_menu_show();
        print!("    PlayTrade <Message Setting>: ");
        player_option_picked.clear();
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut player_option_picked)
        .expect("Failed to read line");

        if !allowed_text.contains(player_option_picked.trim()) {
            println!("    Option not found");
        }

        match player_option_picked.trim() {
            "1" => {
                match edit_message() {
                    Ok(_result) => (),
                    Err(e) => println!("ERROR: {}", e),
                }
            },
            "2" => {
                return;
            }
            _ => {},
        }
    }   
}


fn message_setting_menu_show() {
    println!("    -- Current Message --");
    println!("    {}", ini_file_helpers::access_ini_data("Settings", "message"));

    println!(
    "
    [1]. Edit Message.
    [2]. Return.
    "
    );
}

fn edit_message() -> Result<String, std::io::Error> {
    let mut message = String::new();
    print!("    New Message: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut message)?;

    ini_file_helpers::edit_ini_data("Settings", &message, "Message");
    Ok(("").to_string())
}



