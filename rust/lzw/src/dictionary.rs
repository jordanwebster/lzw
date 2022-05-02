use bimap::BiMap;

pub struct Dictionary {
    map: BiMap<Vec<u8>, u32>,
    next_code: u32,
}

impl Dictionary {
    const DICTIONARY_SIZE: u32 = 4096;

    pub fn new() -> Dictionary {
        let mut dictionary = Dictionary {
            map: BiMap::new(),
            next_code: 0,
        };
        dictionary.init();
        dictionary
    }
    pub fn add(&mut self, entry: Vec<u8>) {
        if self.contains(&entry) {
            return
        };

        if self.next_code == Dictionary::DICTIONARY_SIZE {
            self.reset();
        }

        self.map.insert(entry, self.next_code);
        self.next_code += 1;
    }

    fn reset(&mut self) {
        self.map = BiMap::new();
        self.next_code = 0;
        self.init();
    }

    fn init(&mut self) {
        for i in 0..=255 {
            self.add(vec![i]);
        }
    }

    pub fn contains(&self, entry: &[u8]) -> bool {
        self.map.contains_left(entry)
    }

    pub fn get_code(&self, entry: &[u8]) -> u32 {
        self.map
            .get_by_left(entry)
            .expect("Entry not found in dictionary")
            .clone()
    }

    pub fn get_entry(&self, code: u32) -> &Vec<u8> {
        self.map
            .get_by_right(&code)
            .expect("Code not found in dictionary")
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::Dictionary;

    #[test]
    fn duplicate_add() {
        let mut dictionary = Dictionary::new();

        dictionary.add("hello".as_bytes().to_vec());
        let first_code = dictionary.get_code("hello".as_bytes());

        dictionary.add("hello".as_bytes().to_vec());
        let second_code = dictionary.get_code("hello".as_bytes());

        assert_eq!(first_code, second_code);
    }

    #[test]
    fn reset_dictionary() {
        let mut dictionary = Dictionary::new();
        for i in 256..Dictionary::DICTIONARY_SIZE {
            dictionary.add(format!("entry{}", i).as_bytes().to_vec());
        }
        assert!(dictionary.contains(&"entry256".as_bytes().to_vec()));
        assert!(!dictionary.contains(&"hello world".as_bytes().to_vec()));

        dictionary.add("hello world".as_bytes().to_vec());
        let code = dictionary.get_code("hello world".as_bytes());

        assert_eq!(code, 256);
        assert!(!dictionary.contains(&"entry256".as_bytes().to_vec()));
    }
}
