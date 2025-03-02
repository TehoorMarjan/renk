pub trait Writer {
    fn write(&self, content: &str) -> std::io::Result<()>;
}

mod file_writer;
mod stdout_writer;

pub use file_writer::FileWriter;
pub use stdout_writer::StdoutWriter;
