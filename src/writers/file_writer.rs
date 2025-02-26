use std::io::Write;
use std::fs::File;

pub struct FileWriter {
    path: String,
}

impl FileWriter {
    pub fn new(path: &str) -> Self {
        FileWriter { path: path.to_string() }
    }
}

impl super::Writer for FileWriter {
    fn write(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
