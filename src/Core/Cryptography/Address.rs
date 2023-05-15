use crate::Core::Numerics::Base58::Base58;

use super::{AddressKind::AddressKind, IKeyPair::IKeyPair, PhantasmaKeys::PhantasmaKeys};

pub struct Address {
    bytes: Vec<u8>,
    Kind: AddressKind,
    Text: Option<String>,
}

impl Address {
    const LENGTH_IN_BYTES: usize = 34; // Update the length as per your implementation
    const NullText: &str = "NULL";
    //const Null: Address = Address::new(&[0; Address::LENGTH_IN_BYTES]);
    pub fn Null() -> Address {
        Address::new(&[0; Address::LENGTH_IN_BYTES])
    }

    pub fn IsSytem(&mut self) -> bool {
        return self.Kind == AddressKind::System;
    }

    pub fn IsUser(&mut self) -> bool {
        return self.Kind == AddressKind::User;
    }

    pub fn IsInterop(&mut self) -> bool {
        return self.Kind == AddressKind::Interop;
    }

    pub fn new(public_key: &[u8]) -> Self {
        if public_key.len() != Self::LENGTH_IN_BYTES {
            panic!("publicKey length must be {}", Self::LENGTH_IN_BYTES);
        }
        let mut bytes = vec![0; Self::LENGTH_IN_BYTES];
        bytes.copy_from_slice(public_key);
        Address {
            bytes,
            Kind: AddressKind::User,
            Text: Some(Self::NullText.to_string()),
        }
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Address {
        let mut address = Address::new(&bytes);
        return address;
    }

    pub fn from_text(text: &str) -> Address {
        let mut address = Address::parse(&text);
        address.Text = Some(text.to_string());
        return address;
    }

    pub fn from_key(key: PhantasmaKeys) -> Address {
        let mut bytes = vec![0; Self::LENGTH_IN_BYTES];
        bytes[0] = AddressKind::User as u8;

        match key.public_key().len() {
            32 => bytes[2..34].copy_from_slice(key.public_key()),
            33 => bytes[1..34].copy_from_slice(key.public_key()),
            _ => panic!("Invalid public key length"),
        }
        Address {
            bytes,
            Kind: AddressKind::User,
            Text: None,
        }
    }

    pub fn parse(mut text: &str) -> Address {
        if (text == Self::NullText) {
            return Self::Null();
        }
        let original_text = String::from(text);
        let prefix = text.chars().next().unwrap();
        text = &text[1..];
        let bytes = Base58::decode(text);
        if bytes.len() != Self::LENGTH_IN_BYTES {
            panic!("Invalid address data");
        }

        let mut addr = Self::new(&bytes);
        return addr;
    }
}
