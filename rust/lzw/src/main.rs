mod decompress;
mod dictionary;
mod io;

use crate::decompress::decompress;
use crate::dictionary::Dictionary;
use crate::io::LZWWriter;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = File::open(&args[2]).expect("Failed to open input file");
    let input = BufReader::new(f);

    let f = File::create(&args[3]).expect("Failed to open output file");
    let output = BufWriter::new(f);

    let command = &args[1];
    match command.as_str() {
        "compress" => compress(input, output),
        "decompress" => decompress(input, output),
        cmd => println!("Unrecognised command: {}", cmd),
    }
}

fn compress(input: BufReader<File>, output: BufWriter<File>) {
    let mut dictionary = Dictionary::new();
    let mut buf = LZWWriter::new(output);

    let mut string: Vec<u8> = vec![];

    for byte in input.bytes() {
        let byte = byte.unwrap();
        string.push(byte);

        if !dictionary.contains(&string) {
            let code = dictionary.get_code(&string[0..string.len() - 1]);
            buf.write_code(code);

            dictionary.add(string);
            string = vec![byte];
        }
    }

    if string.len() > 0 {
        let code = dictionary.get_code(&string);
        buf.write_code(code);
    }

    buf.flush();
}
