use crate::cli::Command;
use crate::objects::blob;
use camino::Utf8PathBuf;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    io::{self, Read, Write},
    path::Path,
};

pub fn execute(command: Command) {
    if let Command::HashObject {
        path,
        stdin,
        write_flag,
    } = command
    {
        match (path, stdin) {
            (Some(path), false) => hash_file(path, write_flag),
            (None, true) => hash_stdin(write_flag),
            _ => eprintln!("Invalid arguments"),
        }
    }
}

fn hash_file(path: Utf8PathBuf, write_flag: bool) {
    match std::fs::read(&path) {
        Ok(data) => {
            let data = attach_blob_header(&data);
            let sha1 = compute_blob_sha1(&data);
            if write_flag {
                write_blob_object(&sha1, &data);
            }
            println!("{}", sha1);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn write_blob_object(hash: &str, data: &[u8]) {
    // zlib compress the data
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    match encoder.write_all(data) {
        Ok(_) => (),
        Err(e) => eprintln!("Error compressing object: {}", e),
    }
    match encoder.finish() {
        Ok(compressed_data) => {
            let output_path = blob::get_file_path(&hash);
            if let Some(parent) = Path::new(&output_path).parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("Error creating directory: {}", e);
                }
            }
            match std::fs::write(output_path.clone(), compressed_data) {
                Ok(_) => println!("Wrote blob object to {}", output_path),
                Err(e) => eprintln!("Error writing object to {} : {}", output_path, e),
            }
        }
        Err(e) => eprintln!("Error finishing compression: {}", e),
    }
}

fn hash_stdin(write_flag: bool) {
    let mut buffer = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_size) => {
            let data = attach_blob_header(&buffer);
            let sha1 = compute_blob_sha1(&data);
            if write_flag {
                write_blob_object(&sha1, &data);
            }
            println!("{}", sha1);
        }
        Err(e) => eprintln!("Error reading stdin: {}", e),
    }
}

fn attach_blob_header(data: &[u8]) -> Vec<u8> {
    let mut res = format!("blob {}\0", data.len()).into_bytes();
    res.extend_from_slice(data);
    res
}

fn compute_blob_sha1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_object_sha1() {
        let data = b"hello world\n";
        let data = attach_blob_header(data);
        let sha1 = compute_blob_sha1(&data);
        // result from `echo 'hello world' | git hash-object --stdin`
        assert_eq!(sha1, "3b18e512dba79e4c8300dd08aeb37f8e728b8dad");
    }
}
