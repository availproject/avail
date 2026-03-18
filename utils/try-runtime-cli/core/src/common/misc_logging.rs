use log::{log, Level};
use paris::formatter::colorize_string;

use crate::LOG_TARGET;

fn level_to_color(level: Level) -> &'static str {
    match level {
        Level::Info => "blue",
        Level::Warn => "yellow",
        Level::Error => "red",
        _ => "white",
    }
}

/// A BIG log that's very difficult to miss.
pub fn basti_log(level: Level, message: &str) {
    let color = level_to_color(level);
    log!(
        target: LOG_TARGET,
        level,
        "{}",
        colorize_string(format!(
            "<bold><{}>{}\n\n",
            &color,
            "-".repeat(message.len())
        ))
    );
    log!(
        target: LOG_TARGET,
        level,
        "{}",
        colorize_string(format!("<bold><{}>{}\n\n", &color, message))
    );
    log!(
        target: LOG_TARGET,
        level,
        "{}",
        colorize_string(format!(
            "<bold><{}>{}\n\n",
            &color,
            "-".repeat(message.len())
        ))
    );
}

/// Temporarily demote the log level to a specific level and restore on drop.
pub struct LogLevelGuard(log::LevelFilter);
impl LogLevelGuard {
    pub fn new(new_level: log::LevelFilter) -> Self {
        let old_level = log::max_level();
        log::set_max_level(new_level);
        Self(old_level)
    }

    /// Only show errors.
    pub fn only_errors() -> Self {
        Self::new(log::LevelFilter::Error)
    }
}

impl Drop for LogLevelGuard {
    fn drop(&mut self) {
        log::set_max_level(self.0);
    }
}
