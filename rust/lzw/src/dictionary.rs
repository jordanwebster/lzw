use std::collections::HashMap;

pub struct Dictionary {
    map: HashMap<Vec<u8>, u32>,
    next_code: u32,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let mut dictionary = Dictionary {
            map: HashMap::new(),
            next_code: 0,
        };
        for i in 0..=255 {
            dictionary.add(vec![i]);
        }
        dictionary
    }
    pub fn add(&mut self, entry: Vec<u8>) {
        self.map.insert(entry, self.next_code);
        self.next_code += 1;
    }

    pub fn contains(&self, entry: &[u8]) -> bool {
        self.map.contains_key(entry)
    }

    pub fn get_code(&self, entry: &[u8]) -> &u32 {
        self.map.get(entry).expect("Entry not found in dictionary")
    }
}
