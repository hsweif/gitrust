use crate::objects::{self, blob};
use crate::{cli::Command, objects::factory::Object};
use camino::Utf8PathBuf;
use std::io::Read;

pub fn execute(command: Command) {
    if let Command::HashObject {
        path,
        stdin,
        write_flag,
    } = command
    {
        match (path, stdin) {
            (Some(path), false) => handle_file(path, write_flag),
            (None, true) => handle_stdin(write_flag),
            _ => eprintln!("Invalid arguments"),
        }
    }
}

fn handle_file(path: Utf8PathBuf, write_flag: bool) {
    match std::fs::read(&path) {
        Ok(data) => hash_blob_object(data, write_flag),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn handle_stdin(write_flag: bool) {
    let mut buffer = Vec::new();
    match std::io::stdin().read_to_end(&mut buffer) {
        Ok(_size) => hash_blob_object(buffer, write_flag),
        Err(e) => eprintln!("Error reading stdin: {}", e),
    }
}

fn hash_blob_object(data: Vec<u8>, write_flag: bool) {
    match blob::Blob::from_content(data) {
        Ok(object) => {
            if !write_flag {
                let (hash, _data) = objects::io::hash_object_data(&object);
                println!("{}", hash);
                return;
            }
            match objects::io::write_object(&object) {
                Ok(hash) => println!("{}", hash),
                Err(e) => eprintln!("Error writing object: {}", e),
            }
        }
        Err(e) => eprintln!("Error creating blob object: {}", e),
    }
}
