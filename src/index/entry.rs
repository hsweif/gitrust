use std::{error::Error, fmt, fs::File, io::Read, path::Path};

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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sha1 = hex::encode(self.sha1);
        write!(f, "{:o} {} {}", self.mode, sha1, self.path)
    }
}

pub fn load_index() -> Result<Vec<Entry>, Box<dyn Error>> {
    let index_path = Path::new(".git/index");
    let mut file = File::open(index_path)?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Ensure the index file starts with the expected signature
    if &buffer[0..4] != b"DIRC" {
        return Err("Invalid index file signature".into());
    }

    let _version = u32::from_be_bytes(buffer[4..8].try_into()?);
    let num_entries = u32::from_be_bytes(buffer[8..12].try_into()?);

    let mut entries = Vec::new();
    let mut offset = 12;
    for _ in 0..num_entries {
        let ctime = (
            read_u32(&buffer, &mut offset)?,
            read_u32(&buffer, &mut offset)?,
        );
        let mtime = (
            read_u32(&buffer, &mut offset)?,
            read_u32(&buffer, &mut offset)?,
        );
        let dev = read_u32(&buffer, &mut offset)?;
        let inode = read_u32(&buffer, &mut offset)?;
        let mode = read_u32(&buffer, &mut offset)?;
        let uid = read_u32(&buffer, &mut offset)?;
        let gid = read_u32(&buffer, &mut offset)?;
        let file_size = read_u32(&buffer, &mut offset)?;
        let sha1 = read_bytes::<20>(&buffer, &mut offset)?;
        let flags = read_u16(&buffer, &mut offset)?;
        let path_length = (flags & 0x0fff) as usize;
        let path = String::from_utf8_lossy(&buffer[offset..offset + path_length]).to_string();
        offset += path_length;

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
        offset += padding;
    }
    Ok(entries)
}

fn read_u32(buffer: &[u8], offset: &mut usize) -> Result<u32, Box<dyn Error>> {
    let bytes = buffer
        .get(*offset..*offset + 4)
        .ok_or("Index out of bounds")?;
    *offset += 4;
    Ok(u32::from_be_bytes(bytes.try_into()?))
}

fn read_u16(buffer: &[u8], offset: &mut usize) -> Result<u16, Box<dyn Error>> {
    let bytes = buffer
        .get(*offset..*offset + 2)
        .ok_or("Index out of bounds")?;
    *offset += 2;
    Ok(u16::from_be_bytes(bytes.try_into()?))
}

fn read_bytes<'a, const N: usize>(
    buffer: &'a [u8],
    offset: &mut usize,
) -> Result<[u8; N], Box<dyn Error>> {
    let bytes = buffer
        .get(*offset..*offset + N)
        .ok_or("Index out of bounds")?;
    *offset += N;
    Ok(bytes.try_into()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u32() {
        let buffer = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let mut offset = 0;
        assert_eq!(read_u32(&buffer, &mut offset).unwrap(), 1);
        assert_eq!(read_u32(&buffer, &mut offset).unwrap(), 2);
    }

    #[test]
    fn test_read_u32_out_of_bounds() {
        let buffer = [0x00, 0x00, 0x01];
        let mut offset = 0;
        assert!(read_u32(&buffer, &mut offset).is_err());
    }

    #[test]
    fn test_read_u16() {
        let buffer = [0x00, 0x01, 0x00, 0x02];
        let mut offset = 0;
        assert_eq!(read_u16(&buffer, &mut offset).unwrap(), 1);
        assert_eq!(read_u16(&buffer, &mut offset).unwrap(), 2);
    }

    #[test]
    fn test_read_u16_out_of_bounds() {
        let buffer = [0x01];
        let mut offset = 0;
        assert!(read_u16(&buffer, &mut offset).is_err());
    }

    #[test]
    fn test_read_bytes() {
        let buffer = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let mut offset = 0;
        assert_eq!(
            read_bytes::<3>(&buffer, &mut offset).unwrap(),
            [0x00, 0x01, 0x02]
        );
        assert_eq!(
            read_bytes::<3>(&buffer, &mut offset).unwrap(),
            [0x03, 0x04, 0x05]
        );
    }

    #[test]
    fn test_read_bytes_out_of_bounds() {
        let buffer = [0x01];
        let mut offset = 0;
        assert!(read_bytes::<2>(&buffer, &mut offset).is_err());
    }
}
