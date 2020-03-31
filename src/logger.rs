use log::{Metadata, Record};

pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{}:{}:{} - {}",
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args(),
            );
        }
    }

    fn flush(&self) {}
}
