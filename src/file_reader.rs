pub mod buffered_reader {
    use std::fs::File;
    use std::path::Path;
    use std::io::Read;
    use std::io::Seek;
    use std::io::SeekFrom;
    

    // Memory page size
    // $ getconf -a | grep PAGESIZE
    const DEFAULT_BUFFER_SIZE: usize = 4906;

    pub struct FileReader {
        pos: u64,
        buffer: Vec<u8>,
        buffer_size: usize,
        file: File,
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
        let file = File::open(file_path).unwrap();

        return Ok(FileReader {
            pos: 0,
            buffer: *Box::new(vec![0u8; DEFAULT_BUFFER_SIZE]),
            buffer_size: DEFAULT_BUFFER_SIZE,
            file,
        });
    }

    impl FileReader {
        pub fn next(&mut self) -> Result<&[u8], FileReaderError> {
            let metadata = self.file.metadata().unwrap();
            let next_pos = self.pos + (self.buffer_size as u64);
            match self.file.seek(SeekFrom::Start(self.pos)) {
                Ok(_) => {},
                Err(v) => { 
                    return Err(FileReaderError{
                        message: format!("{}", v),
                    });
                },
            }
            let n = self.file.read(&mut self.buffer).unwrap();
            if n > 0 && self.pos < metadata.len().try_into().unwrap() {
                self.pos = next_pos;
                return Ok(&self.buffer[..n]);
            }
            
            return Err(FileReaderError{
                message: "EOF".parse().unwrap(),
            });
        }
    }


}