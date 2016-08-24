use log::{self, Log, LogLevel, LogRecord, LogMetadata};
use term_painter::Color::*;
use term_painter::ToStyle;

pub struct RocketLogger(Level);

pub enum Level {
    /// Only shows errors and warning.
    Critical,
    /// Shows everything except debug and trace information.
    Normal,
    /// Shows everything.
    Debug,
}

impl Level {
    #[inline(always)]
    fn max_log_level(&self) -> LogLevel {
        match *self {
            Level::Critical => LogLevel::Warn,
            Level::Normal => LogLevel::Info,
            Level::Debug => LogLevel::Trace
        }
    }
}

#[macro_export]
macro_rules! log_ {
    ($name:ident: $format:expr) => { log_!($name: $format,) };
    ($name:ident: $format:expr, $($args:expr),*) => {
        $name!(target: "_", $format, $($args),*);
    };
}

#[macro_export]
macro_rules! error_ { ($($args:expr),+) => { log_!(error: $($args),+); }; }
#[macro_export]
macro_rules! info_ { ($($args:expr),+) => { log_!(info: $($args),+); }; }
#[macro_export]
macro_rules! trace_ { ($($args:expr),+) => { log_!(trace: $($args),+); }; }
#[macro_export]
macro_rules! debug_ { ($($args:expr),+) => { log_!(debug: $($args),+); }; }
#[macro_export]
macro_rules! warn_ { ($($args:expr),+) => { log_!(warn: $($args),+); }; }

impl Log for RocketLogger {
    fn enabled(&self, md: &LogMetadata) -> bool {
        md.level() <= self.0.max_log_level()
    }

    fn log(&self, record: &LogRecord) {
        // Print nothing if this level isn't enabled.
        if !self.enabled(record.metadata()) {
            return;
        }

        // In Rocket, we abuse target with value "_" to indicate indentation.
        if record.target() == "_" {
            print!("    {} ", White.paint("=>"));
        }

        use log::LogLevel::*;
        match record.level() {
            Info => println!("{}", Blue.paint(record.args())),
            Trace => println!("{}", Magenta.paint(record.args())),
            Error => {
                println!("{} {}",
                         Red.bold().paint("Error:"),
                         Red.paint(record.args()))
            }
            Warn => {
                println!("{} {}",
                         Yellow.bold().paint("Warning:"),
                         Yellow.paint(record.args()))
            }
            Debug => {
                let loc = record.location();
                println!("{} {}:{}", Cyan.paint("-->"), loc.file(), loc.line());
                println!("{}", Cyan.paint(record.args()));
            }
        }
    }
}

pub fn init(level: Level) {
    let result = log::set_logger(|max_log_level| {
        max_log_level.set(level.max_log_level().to_log_level_filter());
        Box::new(RocketLogger(level))
    });

    if let Err(err) = result {
        println!("Logger failed to initialize: {}", err);
    }
}
