extern crate log;
extern crate time;
use std::io::{ self, BufWriter, Write };
use std::cell::{ RefCell };
use std::thread;
use std::sync::mpsc;

use log::LogMetadata;

struct ConsoleLogger {
    writer: RefCell<BufWriter<io::Stdout>>,
    tx: mpsc::Sender<String>,
    target: String,
    // level: log::LogLevel,
}

impl ConsoleLogger {
    pub fn new(tx: mpsc::Sender<String>, target: String) -> ConsoleLogger {
        ConsoleLogger {
            writer: RefCell::new(BufWriter::new(io::stdout())), tx: tx,
            // level: level,
            target: target,
        }
    }

    pub fn enabled(&self, target: &str) -> bool {
        target.starts_with(&self.target)
    }
}

impl Drop for ConsoleLogger {
    fn drop(&mut self) {
        let _ = self.writer.borrow_mut().flush();
    }
}

unsafe impl Sync for ConsoleLogger {}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        self.enabled(metadata.target())
    }

    fn log(&self, record: &log::LogRecord) {
        if !log::Log::enabled(self, record.metadata()) { return }
        let s = format!(
            "{} {:<5} [{}] {}\n",
            time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap(),
            record.level().to_string(),
            record.location().module_path(),
            record.args());

        let _ = self.tx.send(s);
    }
}

pub fn init(spec: &str) {
    let (tx, rx) = mpsc::channel();

    let _log_thread = thread::spawn(move || {
        let mut io = io::stdout();
        loop {
            let res : String = rx.recv().unwrap();
            let _ = io.write(res.as_bytes());
        }
    });

    let mut parts = spec.split("=");
    let target = if let Some(target) = parts.next() {
        target
    } else {
        println!("invalid log-spec, disabling log");
        return;
    };

    let log_level = parts.next().unwrap_or("INFO");

    let logger = ConsoleLogger::new(tx, target.to_string());

    log::set_logger(|max_log_level| {
        let log_level = log_level.parse().unwrap();
        max_log_level.set(log_level);
        return Box::new(logger);
    }).unwrap();
}
