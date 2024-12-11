use crate::cli::Command;
use sha1::{Digest, Sha1};
use std::io::{self, Read};

pub fn execute(command: Command) {
    if let Command::HashObject { path, stdin } = command {
        match (path, stdin) {
            (Some(_path), false) => eprintln!("Not implemented yet"),
            (None, true) => execute_stdin(),
            _ => eprintln!("Invalid arguments"),
        }
    }
}

fn execute_stdin() {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).unwrap();
    println!("{}", compute_object_sha1(&buffer));
}

fn compute_object_sha1(data: &[u8]) -> String {
    let mut res = format!("blob {}\0", data.len()).into_bytes();
    res.extend_from_slice(data);
    let mut hasher = Sha1::new();
    hasher.update(res);
    let result = hasher.finalize();
    format!("{:x}", result)
}
