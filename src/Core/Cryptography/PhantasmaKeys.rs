use super::{Address::Address, Entropy::Entropy};
use crate::Core::Cryptography::IKeyPair::IKeyPair;
use crate::Core::Numerics::Base58::Base58;
use ed25519_dalek::ed25519;
use ed25519_dalek::{Keypair, Signer};
use secp256k1::{Message, PublicKey, SecretKey};
use std::fmt;

pub struct PhantasmaKeys {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: Address,
}

impl PhantasmaKeys {
    pub const PRIVATE_KEY_LENGTH: usize = 32;

    pub fn new(mut private_key: Vec<u8>) -> PhantasmaKeys {
        println!("private_key.len() = {}", private_key.len());
        if private_key.len() == 64 {
            private_key = private_key[..32].to_vec();
        }

        if private_key.len() != Self::PRIVATE_KEY_LENGTH {
            panic!(
                "private_key should have length {} but has {}",
                Self::PRIVATE_KEY_LENGTH,
                private_key.len()
            );
        }

        let mut private_key_cloned = private_key.clone();
        private_key_cloned.append(&mut vec![0; 32]);
        let public_key = Self::public_key_from_seed(&private_key_cloned);
        let address = Address::from_bytes(&public_key);

        return PhantasmaKeys {
            private_key,
            public_key,
            address: address,
        };

        /*Self {
            private_key: private_key_cloned,
            public_key,
            address: todo!(),
        }*/
    }

    pub fn private_key(&self) -> &Vec<u8> {
        &self.private_key
    }
    pub fn public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn get_private_key(&self) -> &[u8] {
        &self.private_key
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn generate() -> Self {
        let private_key = Entropy::get_random_bytes(Self::PRIVATE_KEY_LENGTH);
        Self::new(private_key)
    }

    pub fn from_wif(wif: &str) -> PhantasmaKeys {
        let mut data = Base58::decode(&wif);

        if data.len() != 34 || data[0] != 0x80 || data[33] != 0x01 {
            //return Err("Invalid WIF format");
        }

        let private_key = data[1..33].to_vec();
        return Self::new(private_key);
    }

    pub fn to_wif(&self) -> String {
        let mut data = vec![0x80];
        data.extend_from_slice(&self.private_key);
        data.push(0x01);
        return Base58::encode(&data).as_str().to_string();
    }

    fn xor(x: &[u8], y: &[u8]) -> Result<Vec<u8>, &'static str> {
        if x.len() != y.len() {
            return Err("Inputs to XOR must have the same length");
        }

        Ok(x.iter().zip(y.iter()).map(|(a, b)| a ^ b).collect())
    }

    /*pub fn sign<F>(
        &self,
        msg: Vec<u8>,
        custom_sign_function: Option<F>,
    ) -> Result<Signature, secp256k1::Error>
    where
        F: Fn(Vec<u8>, Vec<u8>, Vec<u8>) -> Vec<u8>,
    {
        //let private_key = SecretKey::parse(&self.private_key)?;
        //let message = Message::parse(&msg);

        //Ok(sign(&message, &private_key).0)
    }*/

    pub fn public_key_from_seed(private_key: &[u8]) -> Vec<u8> {
        let keypair = Keypair::from_bytes(private_key).unwrap();
        keypair.public.as_bytes().to_vec()
    }
}

/*impl fmt::Display for PhantasmaKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_wif() {
        let wif = "your_wif_string";
        let keys = PhantasmaKeys::from_wif(wif);

        assert_eq!(keys.to_wif(), wif);
    }

    #[test]
    fn test_to_wif() {
        let wif = "your_wif_string";
        let keys = PhantasmaKeys::generate();
        println!("keys.to_wif() = {}", keys.to_wif());
        //assert_eq!(keys.to_wif(), wif);
    }
}
