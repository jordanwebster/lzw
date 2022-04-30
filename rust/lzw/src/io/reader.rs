use std::io::prelude::*;
use std::io::BufReader;

pub enum LZWReadResult {
    Ok(u32),
    Eof,
}

pub struct LZWReader<T: std::io::Read> {
    reader: BufReader<T>,
    buf: [u8; 3],
    is_first_code: bool,
}

impl<T: std::io::Read> LZWReader<T> {
    pub fn new(reader: BufReader<T>) -> LZWReader<T> {
        LZWReader {
            reader,
            buf: [0; 3],
            is_first_code: true,
        }
    }

    pub fn get_code(&mut self) -> LZWReadResult {
        if !self.is_first_code {
            self.is_first_code = true;
            return LZWReadResult::Ok(((self.buf[1] as u32 & 0x0F) << 8) | self.buf[2] as u32);
        } else {
            let bytes_read = self
                .reader
                .read(&mut self.buf)
                .expect("Failed to read input");
            if bytes_read == 0 {
                return LZWReadResult::Eof;
            }

            if bytes_read == 3 {
                // We have buffered 3 bytes so we can skip reading on the next
                // call. Otherwise, we must have read 2 bytes which means on
                // the next call we will attempt to read again and since we'll
                // get 0 bytes we'll return EOF.
                self.is_first_code = false;
            }
            return LZWReadResult::Ok(
                (self.buf[0] as u32) << 4 | ((self.buf[1] as u32 & 0xF0) >> 4),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{LZWReadResult, LZWReader};
    use std::io::BufReader;

    #[test]
    fn four_codes() {
        let input: Vec<u8> = vec![
            0b00010000, 0b00000001, 0b00000001, 0b00010000, 0b00100001, 0b00000011,
        ];
        let output = read_codes(input);
        assert_eq!(output, vec![256, 257, 258, 259]);
    }

    #[test]
    fn three_codes() {
        let input: Vec<u8> = vec![0b00010000, 0b00000001, 0b00000001, 0b00010000, 0b00100000];
        let output = read_codes(input);
        assert_eq!(output, vec![256, 257, 258]);
    }

    fn read_codes(input: Vec<u8>) -> Vec<u32> {
        let reader = BufReader::new(&*input);
        let mut reader = LZWReader::new(reader);

        let mut output = vec![];
        loop {
            match reader.get_code() {
                LZWReadResult::Ok(n) => output.push(n),
                LZWReadResult::Eof => break,
            }
        }

        output
    }
}
