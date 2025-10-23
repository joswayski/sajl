use crossbeam_channel::Sender;
use serde;
use serde::Serialize;

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

struct LogObject {
    log_level: LogLevels,
    message: String,
}
impl Default for LogObject {
    fn default() -> Self {
        Self {
            log_level: LogLevels::Info,
            message: String::from("not set"),
        }
    }
}
pub struct Logger {
    sender: Sender<LogObject>,
}

impl Logger {
    pub fn new(buffer_size: Option<usize>) -> Logger {
        // Create the channel
        let (sender, receiver) =
            crossbeam_channel::bounded::<LogObject>(buffer_size.unwrap_or_default());

        Logger { sender }
    }
}

fn main() {
    let logger = Logger::new(None);
}
