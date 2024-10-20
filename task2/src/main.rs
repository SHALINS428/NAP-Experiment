mod logger;
mod string_manager;
mod math_utils;
mod thread_pool;
mod config;

use logger::Logger;
// use string_manager::StringManager;
use thread_pool::ThreadPool;
use config::Config;
use std::convert::TryInto;

fn main() {
    let config = Config::new(); // 假设你有一个Config结构体来解析命令行参数
    let logger = Logger::new();
    let pool = ThreadPool::new(config.threads);

    for i in 0..config.tasks {
        let logger_clone = logger.clone();
        pool.execute(move || {
            let result = math_utils::factorial(i.try_into().unwrap()); // 确保转换
            logger_clone.log(&format!("Factorial of {} is {}", i, result));
        });
    }

    pool.join(); // 确保join方法是公有的
}
