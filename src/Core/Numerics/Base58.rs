use std::{ops::Div, str::FromStr, vec};

use num_bigint::{BigInt, BigUint, ToBigInt};

pub struct Base58;

impl Base58 {
    const ALPHABET: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    pub fn decode(input: &str) -> Vec<u8> {
        // Decode Base58 string to BigUint
        let mut bi = BigUint::from_str("0").unwrap();
        for (i, c) in input.chars().enumerate() {
            let digit = Self::ALPHABET.find(c).unwrap();
            bi = bi * Self::ALPHABET.len() + digit;
        }

        // Encode BigUint to byte[]
        // Leading zero bytes get encoded as leading `1` characters
        let leading_zero_count = input
            .chars()
            .take_while(|&c| c == Self::ALPHABET.chars().nth(0).unwrap())
            .count();
        let leading_zeros = vec![0; leading_zero_count];
        let bytes_without_leading_zeros = bi.to_bytes_be();
        leading_zeros
            .into_iter()
            .chain(bytes_without_leading_zeros.into_iter())
            .collect()
    }

    pub fn encode(input: &Vec<u8>) -> String {
        // Decode byte[] to BigUint
        let value = BigInt::from_bytes_be(num_bigint::Sign::Plus, input);

        // Encode BigUint to Base58 string
        let mut s = String::new();
        let mut value = value;
        let mut zero: BigInt = BigInt::from(0);

        /*while value.gt(&zero) {
            let (v, r) = value.div(&Self::ALPHABET.len().to_bigint().unwrap());
            value = v;
            s.insert(
                0,
                Self::ALPHABET.chars().nth(r.to_usize().unwrap()).unwrap(),
            );
        }*/

        // Append `1` for each leading 0 byte
        for &byte in input.iter() {
            if byte == 0u8 {
                s.insert(0, Self::ALPHABET.chars().nth(0).unwrap());
            } else {
                break;
            }
        }

        s
    }
}
