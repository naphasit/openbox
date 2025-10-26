use chrono::Utc;
use colored::{Color, Colorize};
use log::{Level, LevelFilter, Log, Metadata, Record};
pub use log::{debug, error, info, trace, warn};
use std::{
    env,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    sync::Mutex,
};

// TODO: change to real path
const LOGS_PATH: &str = "/home/naphasitng/.local/share/openbox/logs";

pub struct Logger {
    file: Mutex<File>,
}

impl Logger {
    pub fn init() {
        //* ===== Log Level =====
        let log_level_env = env::var("LOG_LEVEL").unwrap_or("INFO".to_string());
        let log_level = match log_level_env.as_str() {
            "OFF" => LevelFilter::Off,
            "ERROR" => LevelFilter::Error,
            "WARN" => LevelFilter::Warn,
            "INFO" => LevelFilter::Info,
            "DEBUG" => LevelFilter::Debug,
            "TRACE" => LevelFilter::Trace,
            _ => LevelFilter::Info,
        };

        //* ===== Timestamp =====
        let date = Utc::now().format("%Y-%m-%d_%H:%M:%S");

        //* ===== Log Path =====
        let log_file_name = format!("{date}.log");
        let log_path = Path::new(LOGS_PATH).join(log_file_name);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Failed to open log file");

        let logger: Logger = Logger {
            file: Mutex::new(file),
        };

        //* ===== Setup Logger =====
        log::set_boxed_logger(Box::new(logger))
            .map(|()| log::set_max_level(log_level))
            .expect("Failed to set logger");
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        //* ===== Timestamp =====
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let date_colored = date.bright_black();

        //* ===== Log Level =====
        let level = record.level();
        let level_color = match level {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Debug => Color::Magenta,
            Level::Trace => Color::BrightCyan,
        };
        let level_colored = level.as_str().color(level_color);

        //* ===== Log Source =====
        let source = record.target();
        let source_colored = source.bright_black();

        //* ===== Log Contents =====
        let contents = record.args();

        //* ===== Write/Print =====
        let mut file = self.file.lock().unwrap();

        if self.enabled(record.metadata()) {
            println!(
                "[{}] [{}] [{:<5}]: {}",
                date_colored, source_colored, level_colored, contents
            );
            writeln!(file, "[{}] [{}] [{:<5}]: {}", date, source, level, contents).unwrap();
        }
    }

    fn flush(&self) {}
}
