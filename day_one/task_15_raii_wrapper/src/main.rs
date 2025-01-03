use std::fs::File;
use std::io::{self, Write};

/// RAII wrapper for managing file resources
pub struct FileWrapper {
    file: Option<File>,
}

impl FileWrapper {
    /// Create a new FileWrapper by opening a file
    pub fn new(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file: Some(file) })
    }

    /// Write data to the file
    pub fn write(&mut self, data: &str) -> io::Result<()> {
        if let Some(file) = self.file.as_mut() {
            file.write_all(data.as_bytes())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "File is not available"))
        }
    }
}

impl Drop for FileWrapper {
    /// Release the file resource when the struct goes out of scope
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            if let Err(e) = file.sync_all() {
                eprintln!("Error syncing file: {}", e);
            }
        }
    }
}

/// Main function to demonstrate usage
fn main() -> io::Result<()> {
    {
        let mut file_wrapper = FileWrapper::new("example.txt")?;
        file_wrapper.write("Hello, RAII!")?;
        println!("Data written to the file successfully.");
    } // FileWrapper goes out of scope here, and the file is automatically closed.

    println!("File resource released.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Read};

    #[test]
    fn test_happy_path_write_and_drop() -> io::Result<()> {
        let test_path = "test_happy.txt";

        // Write to file using FileWrapper
        {
            let mut file_wrapper = FileWrapper::new(test_path)?;
            file_wrapper.write("Testing RAII implementation!")?;
        } // FileWrapper goes out of scope here, and the file is automatically closed.

        // Verify file content
        let mut content = String::new();
        let mut file = File::open(test_path)?;
        file.read_to_string(&mut content)?;
        assert_eq!(content, "Testing RAII implementation!");

        // Clean up test file
        fs::remove_file(test_path)?;
        Ok(())
    }

    #[test]
    fn test_unhappy_path_write_without_file() {
        let mut file_wrapper = FileWrapper { file: None };
        let result = file_wrapper.write("This should fail.");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
    }

    #[test]
    fn test_edge_case_empty_write() -> io::Result<()> {
        let test_path = "test_empty.txt";

        // Write empty content to file
        {
            let mut file_wrapper = FileWrapper::new(test_path)?;
            file_wrapper.write("")?;
        }

        // Verify file content is empty
        let mut content = String::new();
        let mut file = File::open(test_path)?;
        file.read_to_string(&mut content)?;
        assert!(content.is_empty());

        // Clean up test file
        fs::remove_file(test_path)?;
        Ok(())
    }

    #[test]
    fn test_large_file_write() -> io::Result<()> {
        let test_path = "test_large.txt";
        let large_data = "A".repeat(10_000);

        // Write large content to file
        {
            let mut file_wrapper = FileWrapper::new(test_path)?;
            file_wrapper.write(&large_data)?;
        }

        // Verify file content
        let mut content = String::new();
        let mut file = File::open(test_path)?;
        file.read_to_string(&mut content)?;
        assert_eq!(content, large_data);

        // Clean up test file
        fs::remove_file(test_path)?;
        Ok(())
    }
}
