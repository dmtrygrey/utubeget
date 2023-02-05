//#[macro_use]
//extern crate log;

use env_logger::filter::{Builder, Filter};
use log::{Log, Metadata, Record, SetLoggerError};

const FILTER_ENV: &str = "LOG_LEVEL";

pub struct Logger {
    inner: Filter,
}

impl Logger {
    fn new() -> Logger {
        let mut builder = Builder::from_env(FILTER_ENV);

        Logger {
            inner: builder.build(),
        }
    }

    pub fn init() -> Result<(), SetLoggerError> {
        let logger = Self::new();

        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        // Check if the record is matched by the logger before logging
        if self.inner.matches(record) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}