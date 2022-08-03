use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Dictionary {
    words: HashMap<String, bool>,
}

impl Dictionary {
    pub fn new(filename: &str) -> Self {
        let mut words = HashMap::new();
        let lines = utils::read_lines(filename).expect("cant read text file");
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(word) = line {
                words.insert(word, true);
            }
        }
        Dictionary { words: words }
    }
    pub fn is_a_word(&self, a: &str) -> bool {
        println!("{}", a);
        let clean = a.trim().to_uppercase();
        let value = &self.words.contains_key(&clean);
        value.clone()
    }
    pub fn list(&self) {
        for w in &self.words {
            println!("{}", w.0);
        }
    }
}
