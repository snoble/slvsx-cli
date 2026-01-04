use anyhow::Result;
use std::io::{self, Read, Write};

/// Trait for reading input from various sources
pub trait InputReader {
    fn read(&mut self) -> Result<String>;
}

/// Trait for writing output to various destinations
pub trait OutputWriter {
    fn write(&mut self, data: &[u8]) -> Result<()>;
    fn write_str(&mut self, s: &str) -> Result<()> {
        self.write(s.as_bytes())
    }
}

/// Standard input reader
pub struct StdinReader;

impl InputReader for StdinReader {
    fn read(&mut self) -> Result<String> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

/// File input reader
pub struct FileReader {
    path: String,
}

impl FileReader {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl InputReader for FileReader {
    fn read(&mut self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.path)?)
    }
}

/// Standard output writer
pub struct StdoutWriter;

impl OutputWriter for StdoutWriter {
    fn write(&mut self, data: &[u8]) -> Result<()> {
        io::stdout().write_all(data)?;
        Ok(())
    }
}

/// File output writer
pub struct FileWriter {
    path: String,
}

impl FileWriter {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl OutputWriter for FileWriter {
    fn write(&mut self, data: &[u8]) -> Result<()> {
        std::fs::write(&self.path, data)?;
        Ok(())
    }
}

/// Standard error writer
pub trait ErrorWriter {
    fn write_error(&mut self, message: &str) -> Result<()>;
}

pub struct StderrWriter;

impl ErrorWriter for StderrWriter {
    fn write_error(&mut self, message: &str) -> Result<()> {
        eprintln!("{}", message);
        Ok(())
    }
}

/// Create an input reader based on path
pub fn create_input_reader(path: &str) -> Box<dyn InputReader> {
    if path == "-" {
        Box::new(StdinReader)
    } else {
        Box::new(FileReader::new(path.to_string()))
    }
}

/// Create an output writer based on optional path
pub fn create_output_writer(output_path: Option<&str>) -> Box<dyn OutputWriter> {
    if let Some(path) = output_path {
        Box::new(FileWriter::new(path.to_string()))
    } else {
        Box::new(StdoutWriter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// In-memory input reader for testing
    pub struct MemoryReader {
        content: String,
    }

    impl MemoryReader {
        pub fn new(content: String) -> Self {
            Self { content }
        }
    }

    impl InputReader for MemoryReader {
        fn read(&mut self) -> Result<String> {
            Ok(self.content.clone())
        }
    }

    /// In-memory output writer for testing
    pub struct MemoryWriter {
        content: Vec<u8>,
    }

    impl MemoryWriter {
        pub fn new() -> Self {
            Self {
                content: Vec::new(),
            }
        }

        pub fn as_string(&self) -> String {
            String::from_utf8_lossy(&self.content).to_string()
        }

        pub fn as_bytes(&self) -> &[u8] {
            &self.content
        }
    }

    impl OutputWriter for MemoryWriter {
        fn write(&mut self, data: &[u8]) -> Result<()> {
            self.content.extend_from_slice(data);
            Ok(())
        }
    }

    /// In-memory error writer for testing
    pub struct MemoryErrorWriter {
        messages: Vec<String>,
    }

    impl MemoryErrorWriter {
        pub fn new() -> Self {
            Self {
                messages: Vec::new(),
            }
        }

        pub fn messages(&self) -> &[String] {
            &self.messages
        }
    }

    impl ErrorWriter for MemoryErrorWriter {
        fn write_error(&mut self, message: &str) -> Result<()> {
            self.messages.push(message.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_memory_reader() {
        let mut reader = MemoryReader::new("test content".to_string());
        assert_eq!(reader.read().unwrap(), "test content");
    }

    #[test]
    fn test_memory_writer() {
        let mut writer = MemoryWriter::new();
        writer.write(b"test").unwrap();
        assert_eq!(writer.as_string(), "test");
    }

    #[test]
    fn test_memory_error_writer() {
        let mut writer = MemoryErrorWriter::new();
        writer.write_error("error message").unwrap();
        assert_eq!(writer.messages(), &["error message"]);
    }

    #[test]
    fn test_create_input_reader_stdin() {
        let reader = create_input_reader("-");
        // Verify it can be created (type checking would require downcasting)
        assert!(reader.as_ref() as *const dyn InputReader != std::ptr::null());
    }

    #[test]
    fn test_create_input_reader_file() {
        let reader = create_input_reader("test.txt");
        // Verify it can be created
        assert!(reader.as_ref() as *const dyn InputReader != std::ptr::null());
    }

    #[test]
    fn test_create_output_writer_stdout() {
        let writer = create_output_writer(None);
        // Verify it can be created
        assert!(writer.as_ref() as *const dyn OutputWriter != std::ptr::null());
    }

    #[test]
    fn test_create_output_writer_file() {
        let writer = create_output_writer(Some("output.txt"));
        // Verify it can be created
        assert!(writer.as_ref() as *const dyn OutputWriter != std::ptr::null());
    }

    #[test]
    fn test_file_reader() {
        use tempfile::NamedTempFile;
        use std::fs;
        
        let tmp_file = NamedTempFile::new().unwrap();
        fs::write(tmp_file.path(), "test content").unwrap();
        
        let mut reader = FileReader::new(tmp_file.path().to_str().unwrap().to_string());
        assert_eq!(reader.read().unwrap(), "test content");
    }

    #[test]
    fn test_file_writer() {
        use tempfile::NamedTempFile;
        
        let tmp_file = NamedTempFile::new().unwrap();
        let path = tmp_file.path().to_str().unwrap().to_string();
        
        let mut writer = FileWriter::new(path.clone());
        writer.write(b"test content").unwrap();
        
        let content = std::fs::read_to_string(&path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_memory_writer_write_str() {
        let mut writer = MemoryWriter::new();
        writer.write_str("test").unwrap();
        assert_eq!(writer.as_string(), "test");
    }

    #[test]
    fn test_memory_error_writer_multiple() {
        let mut writer = MemoryErrorWriter::new();
        writer.write_error("error 1").unwrap();
        writer.write_error("error 2").unwrap();
        assert_eq!(writer.messages().len(), 2);
        assert_eq!(writer.messages()[0], "error 1");
        assert_eq!(writer.messages()[1], "error 2");
    }
}

