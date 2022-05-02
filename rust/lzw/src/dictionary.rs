use bimap::BiMap;

pub struct Dictionary {
    map: BiMap<Vec<u8>, u32>,
    next_code: u32,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let mut dictionary = Dictionary {
            map: BiMap::new(),
            next_code: 0,
        };
        for i in 0..=255 {
            dictionary.add(vec![i]);
        }
        dictionary
    }
    pub fn add(&mut self, entry: Vec<u8>) {
        if self.contains(&entry) {
            return
        };

        self.map.insert(entry, self.next_code);
        self.next_code += 1;
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
}
