use crate::cli::Command;
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
        let path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
        match std::fs::read(&path) {
            Ok(data) => {
                let (object_size, object_type, content) = parse_object(&data);
                if size_flag {
                    println!("{}", object_size);
                } else if contents_flag {
                    println!("{}", content);
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

fn parse_object(data: &[u8]) -> (usize, String, String) {
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
                String::from_utf8_lossy(content).to_string(),
            );
        }
    }
    // Return empty strings if parsing fails
    (0, "".to_string(), "".to_string())
}
