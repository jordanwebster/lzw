use std::io::prelude::*;
use std::io::BufWriter;

pub struct LZWWriter<T: std::io::Write> {
    writer: BufWriter<T>,
    buf: [u8; 3],
    is_first_code: bool,
}

impl<T: std::io::Write> LZWWriter<T> {
    pub fn new(writer: BufWriter<T>) -> LZWWriter<T> {
        LZWWriter {
            writer,
            buf: [0; 3],
            is_first_code: true,
        }
    }

    pub fn write_code(&mut self, code: &u32) {
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

    pub fn flush(&mut self) {
        if !self.is_first_code {
            self.writer
                .write(&self.buf[0..2])
                .expect("Failed to write to output");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::LZWWriter;
    use std::io::BufWriter;

    #[test]
    fn four_codes() {
        let mut output: Vec<u8> = vec![];
        write_codes(&mut output, 256, 260);

        let expected_output: Vec<u8> = vec![
            0b00010000, 0b00000001, 0b00000001, 0b00010000, 0b00100001, 0b00000011,
        ];

        assert_eq!(output, expected_output);
    }

    #[test]
    fn three_codes() {
        let mut output: Vec<u8> = vec![];
        write_codes(&mut output, 256, 259);

        let expected_output: Vec<u8> =
            vec![0b00010000, 0b00000001, 0b00000001, 0b00010000, 0b00100000];

        assert_eq!(output, expected_output);
    }

    fn write_codes(output: &mut Vec<u8>, lower: u32, upper: u32) {
        let writer = BufWriter::new(output);
        let mut buf = LZWWriter::new(writer);
        for i in lower..upper {
            buf.write_code(&i);
        }
        buf.flush();
    }
}
