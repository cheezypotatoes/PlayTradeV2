use configparser::ini::{Ini};


pub fn access_ini_data(category: &str, data: &str) -> String {
    let mut config = Ini::new();
    config.load("config.ini").expect("Config not found");
    let res = config.get(category, data).unwrap();
    res
    
}

pub fn edit_ini_data(category: &str, data: &str, key: &str) {
    let mut config = Ini::new();
    config.load("config.ini").expect("Config not found");
    config.set(category, key, Some(data.to_string()));
    config.write("config.ini").expect("Config failed to write");

}
