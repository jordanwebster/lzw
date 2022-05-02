mod dictionary;
mod io;
mod lzw;

use std::env;
use std::fs::File;
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
        "compress" => lzw::compress(input, output),
        "decompress" => lzw::decompress(input, output),
        cmd => println!("Unrecognised command: {}", cmd),
    }
}

