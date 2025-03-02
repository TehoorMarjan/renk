pub struct StdoutWriter;

impl super::Writer for StdoutWriter {
    fn write(&self, content: &str) -> std::io::Result<()> {
        println!("{}", content);
        Ok(())
    }
}
