pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    Debug,
    Info,
}
pub struct Logger {
    file_name: String,
}
impl Logger {
    pub fn log(&self, message: &str, log_level: LogLevel) {
        let level_str = match log_level {
            LogLevel::Fatal => "FATAL",
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
        };
        println!("{} - {} : {}", level_str, self.file_name, message);
    }

    pub fn new(file_name_param: &str) -> Logger {
        Logger {
            file_name: file_name_param.to_string(),
        }
    }
}
