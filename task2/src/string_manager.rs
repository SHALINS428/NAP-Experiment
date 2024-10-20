pub struct StringManager {
    strings: Vec<String>,
}

impl StringManager {
    pub fn new() -> Self {
        StringManager {
            strings: Vec::new(),
        }
    }

    pub fn add_string(&mut self, s: String) {
        self.strings.push(s);
    }

    pub fn get_longest(&self) -> &String {
        self.strings.iter().max_by_key(|s| s.len()).unwrap()
    }
}
