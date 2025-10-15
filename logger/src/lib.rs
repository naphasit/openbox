use chrono::Utc;
use colored::{Color, Colorize};
use log::{Level, LevelFilter, Log, Metadata, Record, info};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

// TODO: change to real path
const LOGS_PATH: &str = "/home/naphasitng/.local/share/openbox/logs";

pub struct Logger {
    file: Mutex<File>,
}

impl Logger {
    pub fn init() {
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
        let date = Utc::now().format("%Y-%m-%d_%H:%M:%S");
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
        let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let time_colored = time.bright_black();
        let level_color = match record.level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Debug => Color::Magenta,
            Level::Trace => Color::BrightCyan,
        };
        let level = record.level();
        let level_colored = record.level().as_str().color(level_color);
        let args = record.args();

        let mut file = self.file.lock().unwrap();

        if self.enabled(record.metadata()) {
            println!("[{}] [{:<5}]: {}", time_colored, level_colored, args);
            writeln!(file, "[{}] [{:<5}]: {}", time, level, args).unwrap();
        }
    }

    fn flush(&self) {}
}
