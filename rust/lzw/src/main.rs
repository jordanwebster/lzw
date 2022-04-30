use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[1]).expect("Failed to open input file");
    let input = BufReader::new(f);

    let f = File::create(&args[2]).expect("Failed to open output file");
    let output = BufWriter::new(f);

    compress(input, output);
}

fn compress(input: BufReader<File>, output: BufWriter<File>) {
    let mut dictionary = Dictionary::new();
    let mut buf = OutputBuffer::new(output);

    let mut string: Vec<u8> = vec![];

    for byte in input.bytes() {
        let byte = byte.unwrap();
        string.push(byte);

        if !dictionary.contains(&string) {
            let code = dictionary.get_code(&string[0..string.len() - 1]);
            buf.write(code);

            dictionary.add(string);
            string = vec![byte];
        }
    }

    if string.len() > 0 {
        let code = dictionary.get_code(&string);
        buf.write(code);
    }

    buf.flush();
}

struct OutputBuffer {
    writer: BufWriter<File>,
    buf: [u8; 3],
    is_first_code: bool,
}

impl OutputBuffer {
    fn new(writer: BufWriter<File>) -> OutputBuffer {
        OutputBuffer {
            writer,
            buf: [0; 3],
            is_first_code: true,
        }
    }

    fn write(&mut self, code: &u32) {
        if self.is_first_code {
            self.buf[0] = ((code & 0x0FF0) >> 4) as u8;
            self.buf[1] = ((code & 0x000F) << 4) as u8;
            self.is_first_code = false;
        } else {
            self.buf[1] |= ((code & 0x0F00) >> 8) as u8;
            self.buf[2] = (code & 0x00FF) as u8;
            self.writer
                .write(&self.buf)
                .expect("Failed to write to output");
            self.is_first_code = true;
        }
    }

    fn flush(&mut self) {
        if !self.is_first_code {
            self.writer
                .write(&self.buf[0..2])
                .expect("Failed to write to output");
        }
    }
}

struct Dictionary {
    map: HashMap<Vec<u8>, u32>,
    next_code: u32,
}

impl Dictionary {
    fn new() -> Dictionary {
        let mut dictionary = Dictionary {
            map: HashMap::new(),
            next_code: 0,
        };
        for i in 0..=255 {
            dictionary.add(vec![i]);
        }
        dictionary
    }
    fn add(&mut self, entry: Vec<u8>) {
        self.map.insert(entry, self.next_code);
        self.next_code += 1;
    }

    fn contains(&self, entry: &[u8]) -> bool {
        self.map.contains_key(entry)
    }

    fn get_code(&self, entry: &[u8]) -> &u32 {
        self.map.get(entry).expect("Entry not found in dictionary")
    }
}
