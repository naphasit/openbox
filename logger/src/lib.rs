use chrono::Utc;
use colored::{Color, Colorize};
use log::{Level, Log, Metadata, Record};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::max()
    }

    fn log(&self, record: &Record) {
        let time = Utc::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .bright_black();
        let level_color = match record.level() {
            Level::Error => Color::Red,
            Level::Warn => Color::Yellow,
            Level::Info => Color::Blue,
            Level::Debug => Color::Magenta,
            Level::Trace => Color::BrightCyan,
        };
        let level = record.level().as_str().color(level_color);
        let args = record.args();

        if self.enabled(record.metadata()) {
            println!("[{}] [{:<5}]: {}", time, level, args);
        }
    }

    fn flush(&self) {}
}
