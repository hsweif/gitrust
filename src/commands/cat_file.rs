use crate::cli::Command;
use crate::objects::{blob, factory};
use std::str;

pub fn execute(command: Command) {
    if let Command::CatFile {
        hash,
        type_flag,
        contents_flag,
        size_flag,
    } = command
    {
        let path = blob::get_file_path(&hash);
        match std::fs::read(&path) {
            Ok(data) => {
                let object = factory::parse_object(&data);
                if size_flag {
                    println!("{}", object.get_object_size());
                } else if contents_flag {
                    let content = object.get_content();
                    match str::from_utf8(&content) {
                        Ok(s) => print!("{}", s),
                        Err(_) => eprintln!("Invalid UTF-8 in object"),
                    }
                } else if type_flag {
                    println!("{}", object.get_object_type());
                } else {
                    eprintln!("No flags provided");
                }
            }
            Err(e) => eprintln!("Error reading object: {}", e),
        }
    }
}
