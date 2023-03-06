pub mod buffered_reader {
    use std::path::Path;

    const DEFAULT_BUFFER_SIZE: i32 = 1024;

    pub struct FileReader {
        pos: i32,
        file_path: String,
        buffer_size: i32,
    }

    pub fn create(file_name: &String) -> FileReader {
        return FileReader {
            pos: 0,
            file_path: file_name.clone(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        };
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