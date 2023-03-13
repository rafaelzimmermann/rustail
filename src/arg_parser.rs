pub mod parser {
    use std::env;
    use std::ffi::OsStr;
    use std::path::Path;

    pub fn file_paths() -> Vec<String> {
        return env::args().skip(1).collect();
    }

    pub fn app_name() -> Option<String> {
        return env::args()
            .next()
            .as_ref()
            .map(Path::new)
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .map(String::from);
    }
}
