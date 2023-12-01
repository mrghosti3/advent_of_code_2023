use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("Missing input file name!");

    let mut fopen = BufReader::new(File::open(fname).expect("Error opening file."));

    let mut sum: u32 = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(256);
    while let Ok(read) = fopen.read_until(b'\n', &mut buf) {
        if read == 0 {
            break;
        }
        let mut req_digits = [Option::None, Option::None];

        // first pass from start:
        for (_i, b) in buf.iter().enumerate() {
            if b.is_ascii_digit() && req_digits[0].is_none() {
                req_digits[0] = Some(u8::from_ascii(b));
                break;
            }
        }

        for (_i, b) in buf.iter().enumerate().rev() {
            if b.is_ascii_digit() && req_digits[1].is_none() {
                req_digits[1] = Some(u8::from_ascii(b));
                break;
            }
        }

        let req_digits = [req_digits[0].unwrap_or(0), req_digits[1].unwrap_or(0)];
        sum += req_digits[0] as u32 * 10 + req_digits[1] as u32;
        buf.clear();
    }

    println!("{}", sum);
}

const DIGIT_OFFSET: u8 = 0x30;

trait FromAscii {
    #[inline]
    fn from_ascii(val: &u8) -> u8 {
        val - DIGIT_OFFSET
    }
}

impl FromAscii for u8 {}
