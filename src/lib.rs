use std::io::{Write, stderr};

use chrono::Utc;
use owo_colors::OwoColorize;
use serde;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc::error::TrySendError;
use tokio::time::{self};

// Size of the channel - max messages you can send at one time
const DEFAULT_BUFFER_SIZE: usize = 1024;

// How many log messages to batch
const BATCH_SIZE: usize = 50;

// How long to batch or
const BATCH_DURATION_MS: u64 = 50;

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum LogLevels {
    Info,
    Warn,
    Error,
    Debug,
}

pub struct LoggerOptions {
    /// How many messages to send down the channel before
    /// messages start to be dropped.
    ///
    /// Deault is [`DEFAULT_BUFFER_SIZE`] - 1024
    buffer_size: Option<usize>,

    /// How many log messages to batch
    ///
    /// Default is [`BATCH_SIZE`] - 50
    batch_size: Option<usize>,

    /// For how long to batch messages for
    ///
    /// Default is [`BATCH_DURATION_MS`]
    batch_duration_ms: Option<u64>,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            buffer_size: Some(DEFAULT_BUFFER_SIZE),
            batch_size: Some(BATCH_SIZE),
            batch_duration_ms: Some(BATCH_DURATION_MS),
        }
    }
}

fn get_current_time() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}
#[derive(Serialize)]
struct LogObject {
    log_level: LogLevels,
    data: Value,
    timestamp: String,
}
impl Default for LogObject {
    fn default() -> Self {
        Self {
            log_level: LogLevels::Info,
            data: serde_json::to_value(String::from("not set")).unwrap(),
            timestamp: get_current_time(),
        }
    }
}
pub struct Logger {
    log_sender: tokio::sync::mpsc::Sender<LogObject>,
}

impl Logger {
    pub fn new(options: Option<LoggerOptions>) -> Logger {
        let options = options.unwrap_or_default();
        let buffer_size = options.buffer_size.unwrap_or_default();
        let batch_size = options.batch_size.unwrap_or_default();
        let batch_duration = options.batch_duration_ms.unwrap_or_default();

        let (log_sender, mut log_receiver) = tokio::sync::mpsc::channel::<LogObject>(buffer_size);

        tokio::spawn(async move {
            let mut batch = Vec::<LogObject>::with_capacity(batch_size);
            let mut flush_interval =
                tokio::time::interval(time::Duration::from_millis(batch_duration));
            flush_interval.tick().await; // Skip the first tick which ahppens instantly

            loop {
                tokio::select! {
                    result =   log_receiver.recv() => {
                        match result {
                            Some(log) => {
                                // Found a log!
                                batch.push(log);
                                if batch.len() >= batch_size {
                                    flush_batch(&batch);
                                    batch.clear();
                                }
                            }
                          None => {
                                // Channel closed, flush before dropping
                                if !batch.is_empty() {
                                    flush_batch(&batch);
                                }
                                break;
                            }
                        }
                    }
                    _ = flush_interval.tick() => {
                        // Close and flush at every interval
                        if !batch.is_empty() {
                            flush_batch(&batch);
                            batch.clear();
                        }
                    }
                }
            }
        });

        Logger { log_sender }
    }

    fn log<T: Serialize>(self: &Self, data: &T, log_level: LogLevels) {
        let value = match serde_json::to_value(data) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to serialize {}", e);
                return;
            }
        };

        let x = LogObject {
            log_level,
            data: value,
            timestamp: get_current_time(),
        };

        match self.log_sender.try_send(x) {
            Ok(_) => {}
            Err(e) => match e {
                TrySendError::Full(_) => eprintln!("CHANNEL IS FULL"),
                TrySendError::Closed(_) => eprintln!("CHANNEL CLOSED"),
            },
        }
    }
    pub fn info<T: Serialize>(self: &Self, data: &T) {
        self.log(data, LogLevels::Info);
    }

    pub fn error<T: Serialize>(self: &Self, data: &T) {
        self.log(data, LogLevels::Error);
    }

    pub fn warn<T: Serialize>(self: &Self, data: &T) {
        self.log(data, LogLevels::Warn);
    }

    pub fn debug<T: Serialize>(self: &Self, data: &T) {
        self.log(data, LogLevels::Debug);
    }
}

impl Drop for Logger {
    // Attempt to flush, best effort
    fn drop(&mut self) {
        let _ = std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn flush_batch(batch: &[LogObject]) {
    // Lock once for the whole batch
    let mut stderr = stderr().lock();
    for log in batch {
        let with_color = match log.log_level {
            LogLevels::Error => "ERROR".truecolor(255, 0, 0),
            LogLevels::Info => "INFO".truecolor(15, 115, 255),
            LogLevels::Warn => "WARN".truecolor(247, 155, 35),
            LogLevels::Debug => "DEBUG".truecolor(38, 45, 56),
        };
        writeln!(
            stderr,
            r#"{{"level":"{}","timestamp": {}, "data":{}}}"#,
            with_color,
            log.timestamp,
            serde_json::to_string(&log.data).unwrap()
        )
        .ok();
    }
}
