use std::sync::{Arc, Mutex};

/// 日志记录器结构体
#[derive(Clone)]
pub struct Logger {
    logs: Arc<Mutex<Vec<String>>>, // 使用Arc和Mutex实现线程安全
}

impl Logger {
    /// 创建一个新的Logger实例
    pub fn new() -> Self {
        Logger {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 记录日志信息
    pub fn log(&self, message: &str) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(message.to_string());
        println!("{}", message);
    }

     /// 获取所有日志信息
    pub fn get_logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }
}
