use crate::utils;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ComputerDictionary {
    words: HashMap<String, bool>,
}

pub trait DictionaryLike {
    fn is_a_word(&self, word: &str) -> bool;
}

impl ComputerDictionary {
    pub fn new(filename: &str) -> Self {
        let mut words = HashMap::new();
        let lines = utils::read_lines(filename).expect("cant read text file");
        for line in lines {
            if let Ok(word) = line {
                words.insert(word, true);
            }
        }
        ComputerDictionary { words: HashMap::new() }
    }
    pub fn list(&self) {
        for w in &self.words {
            println!("{}", w.0);
        }
    }
}

impl DictionaryLike for ComputerDictionary {
    fn is_a_word(&self, a: &str) -> bool {
        println!("{}", a);
        let clean = a.trim().to_uppercase();
        let value = &self.words.contains_key(&clean);
        value.clone()
    }
}

#[derive(Debug, Clone)]
pub struct WebDictionary {
    words: HashMap<String, bool>,
}

impl WebDictionary {
    pub fn new() -> Self {
        WebDictionary { words: HashMap::new() }
    }
}

impl DictionaryLike for WebDictionary {
    fn is_a_word(&self, a: &str) -> bool {
        true
    }
}
