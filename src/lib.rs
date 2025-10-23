use serde;
use serde::Serialize;
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::TrySendError;

const DEFAULT_BUFFER_SIZE: usize = 1024;

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
enum LogLevels {
    Info,
    Warn,
    Error,
    Debug,
}

struct LoggerOptions {
    /// How many messages to send down the channel before
    /// messages start to be dropped.
    ///
    /// Deault is [`DEFAULT_BUFFER_SIZE`] - 1024
    buffer_size: Option<usize>,
}

impl Default for LoggerOptions {
    fn default() -> Self {
        Self {
            buffer_size: Some(DEFAULT_BUFFER_SIZE),
        }
    }
}

#[derive(Serialize)]
struct LogObject {
    log_level: LogLevels,
    message: Value,
}
impl Default for LogObject {
    fn default() -> Self {
        Self {
            log_level: LogLevels::Info,
            message: serde_json::to_value(String::from("not set")).unwrap(),
        }
    }
}
pub struct Logger {
    sender: Sender<LogObject>,
}

impl Logger {
    // Create the channel
    pub fn new(options: Option<LoggerOptions>) -> Logger {
        let options = options.unwrap_or_default();
        let buffer_size = options.buffer_size.unwrap_or_default();
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<LogObject>(buffer_size);

        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Some(log) => {
                        let x = json!({"level": log.log_level, "message": log.log_level });
                        println!("{}", x)
                    }
                    None => {}
                }
            }
        });
        Logger { sender }
    }

    pub fn send<T: Serialize>(self: Self, value: serde_json::Value) {
        let x = LogObject {
            log_level: LogLevels::Info,
            message: value,
        };

        match self.sender.try_send(x) {
            Ok(_) => {}
            Err(e) => match e {
                TrySendError::Full(_) => eprintln!("CHANNEL IS FULL"),
                TrySendError::Closed(_) => eprintln!("CHANNEL CLOSED"),
            },
        }
    }
}

fn main() {
    let logger = Logger::new(None);
}
