use std::collections::HashMap;

pub struct Trie {
    pub finish: bool,
    pub childs: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            finish: false,
            childs: HashMap::new(),
        }
    }

    pub fn add_word(&mut self, word: &str) {
        let mut curr: &mut Trie = self;

        for letter in word.chars() {
            curr = curr.childs.entry(letter).or_insert(Trie {
                finish: false,
                childs: HashMap::new(),
            });
        }

        curr.finish = true;
    }

    pub fn add_words(&mut self, words: Vec<&str>) {
        for word in words {
            self.add_word(word);
        }
    }

    pub fn has_word(&mut self, word: &str) -> bool {
        let mut curr: &mut Trie = self;

        for letter in word.chars() {
            match curr.childs.get_mut(&letter) {
                Some(next) => curr = next,
                None => return false,
            }
        }

        return curr.finish;
    }

    pub fn has_partial(&mut self, partial: &str) -> bool {
        let mut curr: &mut Trie = self;

        for letter in partial.chars() {
            match curr.childs.get_mut(&letter) {
                Some(next) => curr = next,
                None => return false,
            }
        }

        return true;
    }
}
