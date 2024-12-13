use std::collections::HashMap;

use super::factory::Object;

#[allow(dead_code)]
pub struct Tree {
    entries: Vec<TreeEntry>,
    index: HashMap<String, usize>,
}

pub struct TreeEntry {
    mode: u32,
    sha1: [u8; 20],
    name: String,
}

impl Object for Tree {
    fn get_content(&self) -> Vec<u8> {
        let mut content = Vec::new();
        for entry in &self.entries {
            // TODO: add object type, which need to read from the DB
            let entry_str = format!(
                "{:06} {} {}\n",
                entry.mode,
                hex::encode(entry.sha1),
                entry.name
            );
            content.extend(entry_str.as_bytes());
        }
        content
    }

    fn from_content(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut entries = Vec::new();
        let mut i = 0;
        while i < data.len() {
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
            let sha1 = &data[i..i + 20]; // SHA-1 hash is 20 bytes
            let sha1: [u8; 20] = sha1.try_into().expect("Invalid hash");
            i += 20;
            let filename_str = std::str::from_utf8(filename)?;
            let mode: u32 = std::str::from_utf8(mode)?.parse()?;
            entries.push(TreeEntry {
                mode,
                sha1,
                name: filename_str.to_string(),
            });
        }
        let mut index = HashMap::new();
        for (i, entry) in entries.iter().enumerate() {
            index.insert(entry.name.clone(), i);
        }
        Ok(Tree { entries, index })
    }

    fn get_object_type(&self) -> &str {
        "tree"
    }
}
