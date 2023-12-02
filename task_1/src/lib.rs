use std::io::BufRead;

pub fn sum_numbers<R: BufRead>(input: &mut R) -> u64 {
    let mut sum: u64 = 0;
    let mut buf: Vec<u8> = Vec::with_capacity(256);

    while let Ok(read) = input.read_until(b'\n', &mut buf) {
        if read == 0 {
            break;
        }
        let mut req_digits = [Option::None, Option::None];

        // first pass from start:
        for (i, b) in buf.iter().enumerate() {
            if b.is_ascii_digit() && req_digits[0].is_none() {
                req_digits[0] = Some(match check_for_text_digits(&buf[..i], SelectDigit::First) {
                    Some(digit) => digit,
                    None => u8::from_ascii(b),
                });
                break;
            }
        }

        for (i, b) in buf.iter().enumerate().rev() {
            if b.is_ascii_digit() && req_digits[1].is_none() {
                req_digits[1] = Some(match check_for_text_digits(&buf[i..], SelectDigit::Last) {
                    Some(digit) => digit,
                    None => u8::from_ascii(b),
                });
                break;
            }
        }

        let [d1, d2] = match req_digits {
            [None, None] => [
                check_for_text_digits(&buf[..], SelectDigit::First).unwrap_or(0),
                check_for_text_digits(&buf[..], SelectDigit::Last).unwrap_or(0),
            ],
            [d1, d2] => [d1.unwrap_or(0), d2.unwrap_or(0)],
        };

        eprintln!(
            "{}{}  --  {}",
            d1,
            d2,
            std::str::from_utf8(&buf[..buf.len() - 1]).unwrap(),
        );

        sum += d1 as u64 * 10 + d2 as u64;
        buf.clear();
    }

    sum
}

const DIGIT_OFFSET: u8 = 0x30;

trait FromAscii {
    #[inline]
    fn from_ascii(val: &u8) -> u8 {
        val - DIGIT_OFFSET
    }
}

impl FromAscii for u8 {}

const TEXT_DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug)]
enum SelectDigit {
    First,
    Last,
}

fn check_for_text_digits(text_slice: &[u8], select: SelectDigit) -> Option<u8> {
    let text_slice = std::str::from_utf8(text_slice).ok()?;
    let mut found_digits = Vec::with_capacity(TEXT_DIGITS.len());

    for (i, txt) in TEXT_DIGITS
        .iter()
        .enumerate()
        .filter(|(_i, txt)| txt.len() <= text_slice.len())
    {
        if let Some(pos) = text_slice.find(txt) {
            found_digits.push((pos, i as u8 + 1));
        }
    }

    if found_digits.len() < 1 {
        return None;
    }

    found_digits.sort_by(|(a_pos, _), (b_pos, _)| a_pos.partial_cmp(b_pos).unwrap());
    match select {
        SelectDigit::First => found_digits.get(0).map(|(_, dig)| *dig),
        SelectDigit::Last => found_digits
            .get(found_digits.len() - 1)
            .map(|(_, dig)| *dig),
    }
}
