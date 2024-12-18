use crate::cli::Command;
use crate::objects::{blob, factory};

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
            Ok(data) => match factory::parse_object(&data) {
                Ok(object) => {
                    if size_flag {
                        println!("{}", object.get_object_size());
                    } else if contents_flag {
                        // we can directly print the object because it implements std::fmt::Display
                        print!("{}", object);
                    } else if type_flag {
                        println!("{}", object.get_object_type());
                    } else {
                        eprintln!("No flags provided");
                    }
                }
                Err(e) => eprintln!("Error reading object: {}", e),
            },
            Err(e) => eprintln!("Error reading object: {}", e),
        }
    }
}
