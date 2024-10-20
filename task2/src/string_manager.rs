/// 字符串管理器结构体
pub struct StringManager {
    strings: Vec<String>, // 存储字符串的Vec
}

impl StringManager {
    /// 创建新的StringManager实例
    pub fn new() -> Self {
        StringManager {
            strings: Vec::new(),
        }
    }

    /// 添加字符串到管理器
    pub fn add_string(&mut self, s: String) {
        self.strings.push(s);
    }

    /// 获取最长字符串
    pub fn get_longest(&self) -> &String {
        self.strings.iter().max_by_key(|s| s.len()).unwrap()
    }

    /// 获取所有字符串
    pub fn get_all_strings(&self) -> &Vec<String> {
        &self.strings
    }
}
