pub struct Config {
    pub threads: usize, // 线程数
    pub tasks: usize,   // 任务数
}

impl Config {
    // 构造函数，用于创建 Config 实例
    pub fn new() -> Config {
        // 这里可以使用命令行解析库来获取参数
        // 暂时假设默认值
        Config {
            threads: 4, // 默认线程数
            tasks: 10,   // 默认任务数
        }
    }
}
