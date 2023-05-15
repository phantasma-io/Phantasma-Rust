use std::char;

pub struct Base16;

impl Base16 {
    const HEX_ALPHABET: &'static str = "0123456789ABCDEF";

    pub fn decode(input: &str, allow_exceptions: bool) -> Option<Vec<u8>> {
        if input.is_empty() {
            return Some(vec![]);
        }

        if input.starts_with("0x") {
            return Self::decode(&input[2..], allow_exceptions);
        }

        let mut result = if input.len() % 2 == 0 {
            let mut bytes = vec![0u8; input.len() / 2];
            for (i, byte) in bytes.iter_mut().enumerate() {
                let str = &input[i * 2..(i * 2 + 2)].to_uppercase();
                let a = Self::HEX_ALPHABET.find(str.chars().nth(0)?);
                let b = Self::HEX_ALPHABET.find(str.chars().nth(1)?);

                if a.is_none() || b.is_none() {
                    return None;
                }

                *byte = (a.unwrap() * 16 + b.unwrap()) as u8;
            }

            Some(bytes)
        } else {
            None
        };

        match result {
            Some(_) => result,
            None => {
                if allow_exceptions {
                    panic!("base16.Decode: invalid input");
                }

                None
            }
        }
    }

    pub fn encode(input: &[u8]) -> String {
        if input.is_empty() {
            return String::new();
        }

        let mut chars = vec![' '; input.len() * 2];
        let mut b;

        for (i, &byte) in input.iter().enumerate() {
            b = byte >> 4;
            chars[i * 2] = char::from_digit((55 + b + ((b - 10) >> 31 & -7)) as u32, 16).unwrap();
            b = byte & 0xF;
            chars[i * 2 + 1] =
                char::from_digit((55 + b + ((b - 10) >> 31 & -7)) as u32, 16).unwrap();
        }

        chars.into_iter().collect()
    }
}
