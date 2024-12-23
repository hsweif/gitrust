use super::{blob::Blob, tree::Tree};
use flate2::read::ZlibDecoder;
use std::io::{self, Read};

pub trait Object: std::fmt::Display {
    fn get_content(&self) -> Vec<u8>; // Method to get the content of the object
    fn from_content(content: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized; // Method to create an object from the content

    fn get_object_size(&self) -> usize {
        self.get_content().len()
    }

    fn get_object_type(&self) -> &str;
}

pub fn parse_object(data: &[u8]) -> Result<Box<dyn Object>, Box<dyn std::error::Error>> {
    let decompressed_data = decompress(data)?;
    let header_end = decompressed_data.iter().position(|&b| b == 0).unwrap();
    let header = &decompressed_data[..header_end];
    let content = decompressed_data[header_end + 1..].to_vec();
    let Some(space_pos) = header.iter().position(|&b| b == b' ') else {
        return Err("Invalid header".into());
    };
    let object_type = &header[..space_pos];
    let _size = std::str::from_utf8(&header[space_pos + 1..])?.parse::<usize>()?;
    let object_type = std::str::from_utf8(object_type)?;
    match object_type {
        "blob" => {
            let blob = Blob::from_content(content)?;
            Ok(Box::new(blob))
        }
        "tree" => {
            let tree = Tree::from_content(content)?;
            Ok(Box::new(tree))
        }
        "commit" | "tag" => unimplemented!(),
        _ => Err("Invalid object type".into()),
    }
}

fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}
