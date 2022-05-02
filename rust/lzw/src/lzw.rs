use crate::dictionary::Dictionary;
use crate::io::{LZWReadResult, LZWReader, LZWWriter};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

pub fn compress<R: std::io::Read, W: std::io::Write>(input: BufReader<R>, output: BufWriter<W>) {
    let mut dictionary = Dictionary::new();
    let mut writer = LZWWriter::new(output);

    let mut string: Vec<u8> = vec![];

    for byte in input.bytes() {
        let byte = byte.unwrap();
        string.push(byte);

        if !dictionary.contains(&string) {
            let code = dictionary.get_code(&string[0..string.len() - 1]);
            writer.write_code(code);

            dictionary.add(string);
            string = vec![byte];
        }
    }

    if string.len() > 0 {
        let code = dictionary.get_code(&string);
        writer.write_code(code);
    }

    writer.flush();
}

pub fn decompress<R: std::io::Read, W: std::io::Write>(
    input: BufReader<R>,
    mut output: BufWriter<W>,
) {
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

#[cfg(test)]
mod tests {
    use crate::lzw;
    use std::io::{BufReader, BufWriter};
    use std::str;

    #[test]
    fn hello_world() {
        let input = "hello world";

        let output = compress_and_decompress(input);

        assert_eq!(input, output);
    }
    
    fn compress_and_decompress(input: &str) -> String {
        let buf_reader = BufReader::new(input.as_bytes());
        let mut compressed: Vec<u8> = vec![];
        let buf_writer = BufWriter::new(&mut compressed);
        lzw::compress(buf_reader, buf_writer);

        let buf_reader = BufReader::new(&*compressed);
        let mut output_bytes: Vec<u8> = vec![];
        let buf_writer = BufWriter::new(&mut output_bytes);
        lzw::decompress(buf_reader, buf_writer);

        str::from_utf8(&output_bytes).unwrap().to_string()
    }
}
