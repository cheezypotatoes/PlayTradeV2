use std::fs::File;
use std::fs;
use std::io::{ErrorKind, Error};
use std::io;
use std::io::{Write};




// TODO: Config file check if tampered
fn create_configuration_file(){
    let mut file = File::create("config.ini").expect("Failed to create config.ini");

    writeln!(file, "[Settings]").expect("Failed to write to config.ini");
    writeln!(file, "primary=None").expect("Failed to write to config.ini"); 
    writeln!(file, "message=None").expect("Failed to write to config.ini"); 

    

}

fn check_if_configuration_file_exist() -> io::Result<()>{
    let content = fs::read_to_string("config.ini")?;

    if content.trim().is_empty() {
        return Err(Error::new(ErrorKind::NotFound, "Config Not found"))
    } 

    Ok(())
}

pub fn bootstrap_main() {
    match check_if_configuration_file_exist() {
        Ok(_result) => (),
        Err(_e) => create_configuration_file(),
    }
}

