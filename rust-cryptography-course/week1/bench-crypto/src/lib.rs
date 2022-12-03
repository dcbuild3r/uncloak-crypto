//! A batteries-included library template.
// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// use anyhow::Result;
use rand::{rngs::ThreadRng, CryptoRng, Rng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

const BITS: usize = 2048;

#[derive(Debug, Clone)]
pub struct RSA {
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

impl RSA {
    pub fn new() -> Self {
        let priv_key =
            RsaPrivateKey::new(&mut rand::thread_rng(), BITS).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        Self { priv_key, pub_key }
    }

    pub fn encrypt<R: CryptoRng + RngCore>(&self, data: &[u8], mut rng: R) -> Vec<u8> {
        self.pub_key
            .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
            .expect("failed to encrypt")
        // assert_ne!(&data[..], &enc_data[..]);
    }

    pub fn decrypt(&self, enc_data: &[u8]) -> Vec<u8> {
        self.priv_key
            .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), enc_data)
            .expect("failed to decrypt")
    }
}

impl Default for RSA {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Vigenere<'a> {
    key: &'a [u8],
}

impl<'a> Vigenere<'a> {
    pub fn new(key: &'a str) -> anyhow::Result<Self> {
        Ok(Self {
            key: key.as_bytes(),
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut enc_data = Vec::with_capacity(data.len());
        for (i, byte) in data.iter().enumerate() {
            enc_data.push(byte ^ self.key[i % self.key.len()]);
        }
        Ok(enc_data)
    }

    pub fn decrypt(&self, enc_data: &[u8]) -> anyhow::Result<Vec<u8>> {
        self.encrypt(enc_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vigenere() {
        let vigenere = Vigenere::new("key").unwrap();
        let data = "hello world";
        let enc_data = vigenere.encrypt(data.as_bytes()).unwrap();
        println!("{}", String::from("enc_data: ").as_str(), &enc_data);
        let dec_data = vigenere.decrypt(&enc_data).unwrap();

        assert_eq!(data.as_bytes(), &dec_data[..]);
    }
}
