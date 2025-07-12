use std::collections::HashSet;
use std::collections::HashMap;
use std::io::{self, Write, ErrorKind, Error};
use std::fs::File;
use std::fs;
use serde_json;
use regex::Regex;
use colored::*;


static MAX_CHARACTERS: usize = 10;


pub fn server_setting_main() {
    let allowed_text: HashSet<&str> = HashSet::from(["1", "2", "3", "4"]);
    let mut player_option_picked = String::new(); 

    while player_option_picked.trim() != "4" {
        
        server_setting_menu_show();
        print!("    PlayTrade <Server Setting>: ");
        player_option_picked.clear();
        io::stdout().flush().unwrap();

        io::stdin()
        .read_line(&mut player_option_picked)
        .expect("Failed to read line");

        if !allowed_text.contains(player_option_picked.trim()) {
            println!("\n    {} Option not found", "ERROR:".red().bold());
        }

        match player_option_picked.trim() {
            "1" => {
                match add_server() {
                    Ok(_result) => (),
                    Err(e) => println!("\n    {} {}", "ERROR:".red().bold(), e),
                }
            },
            "2" => {
                match remove_server() {
                    Ok(_result) => (),
                    Err(e) => println!("\n    {} {}", "ERROR:".red().bold(), e),
                }
            },
            "3" => {
                return;
            }
            _ => {},
        }
    }   

    
}

fn add_server() -> Result<String, std::io::Error>  {
    let mut name = String::new();
    let mut server_id = String::new();
    let re = Regex::new(r"^https://discord\.com/api/v\d+/channels/\d{17,20}/messages$").unwrap();


    print!("    Server Name {}:", format!("[LIMIT {} CHARACTERS]", MAX_CHARACTERS).truecolor(192, 192, 0).bold());
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)?;

    if name.trim().is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Name cannot be empty"))
    } else if name.trim().len() > MAX_CHARACTERS { // max character is 10
        return Err(Error::new(ErrorKind::InvalidInput, format!("Name must be {} characters or less", MAX_CHARACTERS),))
    }

   
    print!("    Server Api Link: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut server_id)?;

    if server_id.trim().is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Token cannot be empty"))
    } 
    
    if !re.is_match(&server_id.trim()) {
        return Err(Error::new(ErrorKind::InvalidInput, format!("Invalid Server Format"),))
    }


    
    let mut server_hash = get_hashmap();
    server_hash.insert(name.clone().trim().to_string(), server_id.clone().trim().to_string());
    save_hashmap(server_hash);

    println!("\n\n");
    Ok(format!("Name: {}, Token: {}", name, server_id))

}

fn remove_server() -> Result<String, std::io::Error> {
    let mut key_name = String::new();
    print!("    Server Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut key_name)?;

    
    match remove_server_key_in_json(&key_name) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(e),
    }
}

fn remove_server_key_in_json(key_to_remove: &String) -> Result<String, std::io::Error> {

    let mut json_hash = get_hashmap();
    if !json_hash.contains_key(key_to_remove.trim()) { 
        return Err(Error::new(ErrorKind::InvalidInput, format!("Name not found!"),));
    }
    json_hash.remove(key_to_remove.trim());
    save_hashmap(json_hash);
    Ok("Server removed successfully".to_string())
}




fn server_setting_menu_show() {
    let hash_map = get_hashmap();
   

    println!("    {}", "-- Server Setting --".truecolor(0, 128, 128).bold());
    for (key, value) in &hash_map {
        println!("    {:<width$} - {}", key, value, width = MAX_CHARACTERS);
    }

    println!("    {} Add/Edit Server.", "[1]".truecolor(0, 128, 128).bold());
    println!("    {} Remove Server.", "[2]".truecolor(0, 128, 128).bold());
    println!("    {} Return.", "[3]".truecolor(0, 128, 128).bold());
  
    
}

fn save_hashmap(hashmap: HashMap<String, String>) {
    let json = serde_json::to_string(&hashmap).unwrap();
    let mut file = File::create("server.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

}

fn check_if_json_exist() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("server.json")?;

    if content.trim().is_empty() {
        return Err(Error::new(ErrorKind::UnexpectedEof, "JSON file is empty"));
    }

    Ok("Good".to_string())

}

fn get_hashmap() -> HashMap<String, String> {
    match check_if_json_exist() {
        Ok(_result) => (),
        Err(_e) => save_hashmap(HashMap::new()), // New hashmap if empty or does not exist
    }


    let file_read = File::open("server.json").unwrap();

    
    let map_read: HashMap<String, String> = serde_json::from_reader(file_read).unwrap();
    map_read
}

