use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fs;

use crate::commands::ls_file::{load_index, IndexEntry};

pub fn execute() {
    match load_index() {
        Ok(entries) => match write_tree_from_index(entries) {
            Ok(sha) => println!("{}", sha),
            Err(e) => eprintln!("Write tree error: {}", e),
        },
        Err(e) => eprintln!("Load index error: {}", e),
    }
}

/// Represents a tree entry in Git.
struct TreeEntry {
    mode: String,
    name: String,
    object_id: String, // SHA1 hash of the blob or subtree
}

fn write_tree_from_index(index: Vec<IndexEntry>) -> Result<String, Box<dyn std::error::Error>> {
    let mut tree_data = Vec::new();
    let mut directories: HashMap<String, Vec<TreeEntry>> = HashMap::new();

    for entry in index {
        // Parse mode, name, and blob SHA1 from the index
        let mode = format!("{:o}", entry.mode);
        let name = entry.path.clone();
        let object_id = entry.sha1.clone();
        let path_components: Vec<&str> = name.split('/').collect();
        let object_id = hex::encode(object_id);

        if path_components.len() == 1 {
            // Direct file in the root tree
            tree_data.push(TreeEntry {
                mode,
                name,
                object_id,
            });
        } else {
            // File in a subdirectory
            let dir = path_components[0].to_string();
            let sub_name = path_components[1..].join("/");
            directories
                .entry(dir)
                .or_insert_with(Vec::new)
                .push(TreeEntry {
                    mode,
                    name: sub_name,
                    object_id,
                });
        }
    }

    // Recursively write subtrees and construct the final tree
    for (dir, entries) in directories {
        let subtree_sha = write_tree(entries)?;
        tree_data.push(TreeEntry {
            mode: "040000".to_string(),
            name: dir,
            object_id: subtree_sha,
        });
    }

    // Serialize tree data
    let mut serialized_tree = Vec::new();
    for entry in &tree_data {
        serialized_tree.extend(format!("{} {}\0", entry.mode, entry.name).as_bytes());
        serialized_tree.extend(hex::decode(&entry.object_id)?);
    }

    // Write to Git object database
    let tree_object =
        format!("tree {}\0", serialized_tree.len()) + &String::from_utf8_lossy(&serialized_tree);
    let sha = Sha1::digest(tree_object.as_bytes());
    let sha_hex = hex::encode(sha);

    fs::write(
        format!(".git/objects/{}/{}", &sha_hex[0..2], &sha_hex[2..]),
        tree_object,
    )?;
    Ok(sha_hex)
}

fn write_tree(entries: Vec<TreeEntry>) -> Result<String, Box<dyn std::error::Error>> {
    let mut serialized_tree = Vec::new();

    for entry in entries {
        // Serialize each tree entry: mode, name, and SHA1
        serialized_tree.extend(format!("{} {}\0", entry.mode, entry.name).as_bytes());
        serialized_tree.extend(hex::decode(entry.object_id)?);
    }

    // Construct the tree object
    let tree_object =
        format!("tree {}\0", serialized_tree.len()) + &String::from_utf8_lossy(&serialized_tree);

    // Compute SHA1 hash
    let sha = Sha1::digest(tree_object.as_bytes());
    let sha_hex = hex::encode(sha);

    // Write the object to the Git database
    let object_path = format!(".git/objects/{}/{}", &sha_hex[0..2], &sha_hex[2..]);
    fs::create_dir_all(&format!(".git/objects/{}", &sha_hex[0..2]))?;
    println!("Writing tree object to {}", object_path);
    fs::write(object_path, tree_object)?;

    Ok(sha_hex)
}
