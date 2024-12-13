use super::factory::Object;

#[derive(Debug)]
pub struct Blob {
    content: Vec<u8>,
}

impl Object for Blob {
    fn get_content(&self) -> Vec<u8> {
        self.content.clone()
    }
    fn from_content(content: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Blob { content })
    }
    fn get_object_type(&self) -> &str {
        "blob"
    }
}

pub fn get_file_path(hash: &str) -> String {
    format!(".git/objects/{}/{}", &hash[..2], &hash[2..])
}
