pub struct StdoutWriter;

impl super::Writer for StdoutWriter {
    fn write(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", content);
        Ok(())
    }
}
