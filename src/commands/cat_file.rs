use crate::cli::Command;
use crate::objects::blob;
use flate2::read::ZlibDecoder;
use std::io::Read;
use std::str;

pub fn execute(command: Command) {
    if let Command::CatFile {
        hash,
        type_flag,
        contents_flag,
        size_flag,
    } = command
    {
        // TODO: parse git directory recursively
        let path = blob::get_file_path(&hash);
        match std::fs::read(&path) {
            Ok(data) => {
                let (object_size, object_type, content) = parse_object(&data);
                if size_flag {
                    println!("{}", object_size);
                } else if contents_flag {
                    if object_type == "tree" {
                        print!("{}", parse_tree_object(&content));
                    } else {
                        let content = str::from_utf8(&content).expect("Invalid UTF-8");
                        print!("{}", content);
                    }
                } else if type_flag {
                    println!("{}", object_type);
                } else {
                    eprintln!("No flags provided");
                }
            }
            Err(e) => eprintln!("Error reading object: {}", e),
        }
    }
}

fn parse_object(data: &[u8]) -> (usize, String, Vec<u8>) {
    // 1. Decompress the data with zlib
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder
        .read_to_end(&mut decompressed_data)
        .expect("Failed to decompress data");
    // 2. Parse the decompressed data. Split header and content. Noting that git's header is in the format
    // [object type] [size]\0[content]
    if let Some(null_pos) = decompressed_data.iter().position(|&b| b == 0) {
        let header = &decompressed_data[..null_pos];
        let content = &decompressed_data[null_pos + 1..];
        // 3. Parse the header to get the object type and size
        if let Some(space_pos) = header.iter().position(|&b| b == b' ') {
            let object_type = &header[..space_pos];
            let size = str::from_utf8(&header[space_pos + 1..])
                .expect("Invalid UTF-8 in size")
                .parse::<usize>()
                .expect("Invalid size");
            let object_type_str = str::from_utf8(object_type)
                .expect("Invalid UTF-8 in object type")
                .to_string();
            // Return type and content
            return (
                size,
                object_type_str,
                content.iter().map(|&b| b).collect::<Vec<u8>>(),
            );
        }
    }
    // Return empty strings if parsing fails
    (0, String::new(), Vec::new())
}

fn parse_tree_object(data: &[u8]) -> String {
    let mut i = 0;
    let mut result = String::new();
    while i < data.len() {
        // Read mode
        let mut mode_end = i;
        while mode_end < data.len() && data[mode_end] != b' ' {
            mode_end += 1
        }
        if mode_end == data.len() {
            break;
        }
        let mode = &data[i..mode_end];

        // Read filename
        i = mode_end + 1;
        let mut filename_end = i;
        while data[filename_end] != 0 {
            filename_end += 1;
        }
        let filename = &data[i..filename_end];

        // Read hash
        i = filename_end + 1;
        let hash = &data[i..i + 20]; // SHA-1 hash is 20 bytes
        i += 20;
        // Convert hash to hex
        let hash_hex = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let mode_str = str::from_utf8(mode).expect("Invalid mode");
        let mode: u32 = mode_str.parse().expect("Invalid number");
        let filename_str = str::from_utf8(filename).expect("Invalid filename");
        result.push_str(&format!("{:06} {}  {}\n", mode, hash_hex, filename_str));
    }
    result
}
