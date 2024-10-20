use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Logger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log(&self, message: &str) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(message.to_string());
        println!("{}", message);
    }
}
