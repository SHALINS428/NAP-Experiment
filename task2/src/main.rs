mod logger;
mod string_manager;

use std::thread;
use string_manager::StringManager;
use logger::Logger;

fn main() {
    let logger = Logger::new();

    // 创建一个字符串管理器
    let mut manager = StringManager::new();

    // 添加字符串
    manager.add_string("hello".to_string());
    manager.add_string("world".to_string());
    manager.add_string("rust".to_string());
    manager.add_string("programming".to_string());

    // 获取最长字符串
    let longest = manager.get_longest();
    logger.log(&format!("The longest string is: {}", longest));

    // 创建多个线程
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let logger_clone = logger.clone();
            thread::spawn(move || {
                logger_clone.log(&format!("Thread {} is running", i));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
