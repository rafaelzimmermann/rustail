pub mod buffered_reader {
    use std::fmt::{Debug, Display};
    use std::path::Path;

    const DEFAULT_BUFFER_SIZE: i32 = 1024;

    pub struct FileReader {
        pos: i32,
        file_path: String,
        buffer_size: i32,
    }

    pub struct FileReaderError {
        pub(crate) message: String,
    }

    pub fn create(file_path: &String) -> Result<FileReader, FileReaderError> {
        if !Path::new(file_path).exists() {
            let result = Err(FileReaderError{
                message: "File does not exits.".parse().unwrap(),
            });
            return result
        }
        return Ok(FileReader {
            pos: 0,
            file_path: file_path.clone(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        });
    }

    impl FileReader {
        pub fn next(&mut self) -> &str {
            let next_pos = self.pos + self.buffer_size;
            // Read buffer size
            self.pos = next_pos;
            return "";
        }
    }


}