use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub mod entry;

#[allow(dead_code)]
pub struct Entry {
    ctime: (u32, u32),
    mtime: (u32, u32),
    dev: u32,
    inode: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    sha1: [u8; 20],
    flags: u16,
    path: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sha1 = hex::encode(self.sha1);
        write!(f, "{:o} {} {}", self.mode, sha1, self.path)
    }
}

pub fn load_index() -> io::Result<Vec<Entry>> {
    let index_path = Path::new(".git/index");
    let mut file = File::open(index_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Ensure the index file starts with the expected signature
    if &buffer[0..4] != b"DIRC" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid index file signature",
        ));
    }

    let _version = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
    let num_entries = u32::from_be_bytes(buffer[8..12].try_into().unwrap());

    let mut entries = Vec::new();
    let mut offset = 12;
    for _ in 0..num_entries {
        // TODO: rewrite the offset management
        let ctime = (
            u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap()),
            u32::from_be_bytes(buffer[offset + 4..offset + 8].try_into().unwrap()),
        );
        let mtime = (
            u32::from_be_bytes(buffer[offset + 8..offset + 12].try_into().unwrap()),
            u32::from_be_bytes(buffer[offset + 12..offset + 16].try_into().unwrap()),
        );
        let dev = u32::from_be_bytes(buffer[offset + 16..offset + 20].try_into().unwrap());
        let inode = u32::from_be_bytes(buffer[offset + 20..offset + 24].try_into().unwrap());
        let mode = u32::from_be_bytes(buffer[offset + 24..offset + 28].try_into().unwrap());
        let uid = u32::from_be_bytes(buffer[offset + 28..offset + 32].try_into().unwrap());
        let gid = u32::from_be_bytes(buffer[offset + 32..offset + 36].try_into().unwrap());
        let file_size = u32::from_be_bytes(buffer[offset + 36..offset + 40].try_into().unwrap());
        let sha1 = buffer[offset + 40..offset + 60].try_into().unwrap();
        let flags = u16::from_be_bytes(buffer[offset + 60..offset + 62].try_into().unwrap());
        let path_length = (flags & 0x0fff) as usize;
        let path =
            String::from_utf8_lossy(&buffer[offset + 62..offset + 62 + path_length]).to_string();
        entries.push(Entry {
            ctime,
            mtime,
            dev,
            inode,
            mode,
            uid,
            gid,
            file_size,
            sha1,
            flags,
            path,
        });
        // Move to the next entry, considering padding
        let entry_size = 62 + path_length;
        let padding = 8 - (entry_size % 8);
        offset += entry_size + padding;
    }
    Ok(entries)
}
