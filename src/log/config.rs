#[derive(Default)]
pub struct LogConfig {
    pub enable_file: bool,
    pub log_file: String,
}

impl LogConfig {
    pub fn with_file(file_name: &str) -> Self {
        LogConfig {
            enable_file: true,
            log_file: String::from(file_name),
        }
    }
}
