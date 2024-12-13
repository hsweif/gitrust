use super::{blob::Blob, tree::Tree};
use flate2::read::ZlibDecoder;
use std::io::{self, Read};

pub trait Object {
    fn get_content(&self) -> Vec<u8>; // Method to get the content of the object
    fn from_content(content: Vec<u8>) -> Self
    where
        Self: Sized; // Method to create an object from content

    fn get_object_size(&self) -> usize {
        self.get_content().len()
    }

    fn get_object_type(&self) -> &str;
}

pub fn parse_object(data: &[u8]) -> Box<dyn Object> {
    let decompressed_data = decompress(data).unwrap();
    let header_end = decompressed_data.iter().position(|&b| b == 0).unwrap();
    let header = &decompressed_data[..header_end];
    let content = decompressed_data[header_end + 1..].to_vec();
    if let Some(space_pos) = header.iter().position(|&b| b == b' ') {
        let object_type = &header[..space_pos];
        let _size = std::str::from_utf8(&header[space_pos + 1..])
            .expect("Invalid UTF-8 in size")
            .parse::<usize>()
            .expect("Invalid size");

        let object_type = std::str::from_utf8(object_type).unwrap();
        match object_type {
            "blob" => Box::new(Blob::from_content(content)),
            "tree" => Box::new(Tree::from_content(content)),
            "commit" | "tag" => unimplemented!(),
            _ => panic!("Unknown object type"),
        }
    } else {
        panic!("Invalid object");
    }
}

fn decompress(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}
