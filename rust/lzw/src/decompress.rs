use crate::dictionary::Dictionary;
use crate::io::{LZWReadResult, LZWReader};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub fn decompress(input: BufReader<File>, mut output: BufWriter<File>) {
    let mut dictionary = Dictionary::new();
    let mut reader = LZWReader::new(input);

    let mut prev_bytes: Vec<u8> = vec![];
    loop {
        match reader.get_code() {
            LZWReadResult::Ok(code) => {
                let bytes = dictionary.get_entry(code).to_vec();
                output.write(&bytes).expect("Failed to write to output");
                prev_bytes.push(bytes[0]);
                dictionary.add(prev_bytes);
                prev_bytes = bytes;
            }
            LZWReadResult::Eof => break,
        }
    }

    output.flush().expect("Failed to write to output");
}
