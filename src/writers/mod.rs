use std::io::Write;
use std::fs::File;

pub trait Writer {
    fn write(&self, content: &str) -> Result<(), Box<dyn std::error::Error>>;
}

mod file_writer;
mod stdout_writer;

pub use file_writer::FileWriter;
pub use stdout_writer::StdoutWriter;
