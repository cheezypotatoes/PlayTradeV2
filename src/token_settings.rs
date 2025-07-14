use std::collections::HashSet;
use std::collections::HashMap;
use std::io::{self, Write, ErrorKind, Error};
use std::fs::File;
use std::fs;
use colored::*;
use serde_json;

use crate::ini_file_helpers;



static MAX_CHARACTERS: usize = 10;


pub fn token_setting_main() {
    let allowed_text: HashSet<&str> = HashSet::from(["1", "2", "3", "4"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "4" {
        
        token_setting_menu_show();
        print!("    PlayTrade <Token Setting>: ");
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
                match add_token() {
                    Ok(_result) => (),
                    Err(e) => println!("ERROR: {}", e),
                }
            },
            "2" => {
                match remove_token() {
                    Ok(_result) => (),
                    Err(e) => println!("ERROR: {}", e),
                }
            }
            "3" => {
                match change_primary() {
                    Ok(_result) => (),
                    Err(e) => println!("ERROR: {}", e),
                }
            }
            "4" => {
                return;
            }
            _ => {},
        }
    }   

    
}


fn token_setting_menu_show() {
    let hash_map = get_hashmap();
    let mut primary_found = false;
    let primary = ini_file_helpers::access_ini_data("Settings", "primary");

    println!("\n    {}", "-- Token Setting --".truecolor(0, 128, 128).bold());
    for (key, value) in &hash_map {
        if key == &primary {
            println!("    {:<width$} - {} [Primary]", key, value, width = MAX_CHARACTERS);
            primary_found = !primary_found;
        } else {
            println!("    {:<width$} - {}", key, value, width = MAX_CHARACTERS);
        }


        
    }

    if !primary_found {
        println!("    (No Primary Session Found)");
    }

    
    println!("    {} Add/Edit Token.", "[1]".truecolor(0, 128, 128).bold());
    println!("    {} Remove Token.", "[2]".truecolor(0, 128, 128).bold());
    println!("    {} Set Primary.", "[3]".truecolor(0, 128, 128).bold());
    println!("    {} Return.", "[4]".truecolor(0, 128, 128).bold());

    
    
}


fn add_token() -> Result<String, std::io::Error>  {
    let mut name = String::new();
    let mut token = String::new();

    print!("    Token Name [LIMIT {} CHARACTERS]:", MAX_CHARACTERS);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)?;

    if name.trim().is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Name cannot be empty"))
    } else if name.trim().len() > MAX_CHARACTERS { // max character is 10
        return Err(Error::new(ErrorKind::InvalidInput, format!("Name must be {} characters or less", MAX_CHARACTERS),))
    }

    print!("    Token: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut token)?;

    if token.trim().is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Token cannot be empty"))
    }

    
    let mut token_hash = get_hashmap();
    token_hash.insert(name.clone().trim().to_string(), token.clone().trim().to_string());
    save_hashmap(token_hash);


    Ok(format!("Name: {}, Token: {}", name, token))

}


fn get_hashmap() -> HashMap<String, String> {
    match check_if_json_exist() {
        Ok(_result) => (),
        Err(_e) => save_hashmap(HashMap::new()), // New hashmap if empty or does not exist
    }


    let file_read = File::open("tokens.json").unwrap();

    
    let map_read: HashMap<String, String> = serde_json::from_reader(file_read).unwrap();
    map_read
}

fn save_hashmap(hashmap: HashMap<String, String>) {
    let json = serde_json::to_string(&hashmap).unwrap();
    let mut file = File::create("tokens.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

}

fn check_if_json_exist() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("tokens.json")?;

    if content.trim().is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "JSON file is empty"));
    }

    Ok("Good".to_string())

}

fn change_primary() -> Result<String, std::io::Error> {
    let mut primary_name = String::new();
    print!("    Primary Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut primary_name)?;

    ini_file_helpers::edit_ini_data("Settings", &primary_name, "Primary");
    Ok(("").to_string())
    
}

fn remove_token() -> Result<String, std::io::Error> {
    let mut key_name = String::new();
    print!("    Key Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut key_name)?;

    
    remove_token_key_in_json(&key_name);
    Ok(("").to_string())
}

fn remove_token_key_in_json(key_to_remove: &String) {
    let mut json_hash = get_hashmap();
    json_hash.remove(key_to_remove.trim());
    save_hashmap(json_hash);
}



