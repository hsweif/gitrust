use super::factory::Object;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{error::Error, fs, io::Write, path::Path};

pub fn write_object<T: Object>(object: &T) -> Result<String, Box<dyn Error>> {
    let (hash, data) = hash_object_data(object);

    // zlib compress the data
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    if let Err(e) = encoder.write_all(&data) {
        eprintln!("Error compressing object: {}", e);
    }

    // Create the directory if it doesn't exist
    let output_path = get_file_path(&hash);
    if let Some(parent) = Path::new(&output_path).parent() {
        fs::create_dir_all(parent)?
    }
    // Write the compressed data to the file
    let compressed_data = encoder.finish()?;
    fs::write(output_path, compressed_data)?;
    Ok(hash)
}

pub fn hash_object_data<T: Object>(object: &T) -> (String, Vec<u8>) {
    let header = format!(
        "{} {}\0",
        object.get_object_type(),
        object.get_object_size()
    );
    let mut data = header.into_bytes();
    let content = object.get_content();
    data.extend_from_slice(&content);
    let mut hasher = Sha1::new();
    hasher.update(data.clone());
    let hash = hasher.finalize();
    let hash = format!("{:x}", hash);
    (hash, data)
}

pub fn get_file_path(hash: &str) -> String {
    format!(".git/objects/{}/{}", &hash[..2], &hash[2..])
}
