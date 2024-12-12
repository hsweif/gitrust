use crate::cli::Command;
use camino::Utf8PathBuf;
use sha1::{Digest, Sha1};
use std::io::{self, Read};

pub fn execute(command: Command) {
    if let Command::HashObject { path, stdin } = command {
        match (path, stdin) {
            (Some(path), false) => hash_file(path),
            (None, true) => hash_stdin(),
            _ => eprintln!("Invalid arguments"),
        }
    }
}

fn hash_file(path: Utf8PathBuf) {
    match std::fs::read(&path) {
        Ok(data) => {
            let sha1 = compute_blob_sha1(&data);
            println!("{}", sha1);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn hash_stdin() {
    let mut buffer = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_size) => println!("{}", compute_blob_sha1(&buffer)),
        Err(e) => eprintln!("Error reading stdin: {}", e),
    }
}

fn compute_blob_sha1(data: &[u8]) -> String {
    let mut res = format!("blob {}\0", data.len()).into_bytes();
    res.extend_from_slice(data);
    let mut hasher = Sha1::new();
    hasher.update(res);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_object_sha1() {
        let data = b"hello world\n";
        let sha1 = compute_blob_sha1(data);
        // result from `echo 'hello world' | git hash-object --stdin`
        assert_eq!(sha1, "3b18e512dba79e4c8300dd08aeb37f8e728b8dad");
    }
}
