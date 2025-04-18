use aead::{Aead, KeyInit};
use chacha20poly1305::{XChaCha20Poly1305, XNonce, Key}; // ou aes-gcm
use std::error::Error;

pub struct CryptoEngine {
    cipher: XChaCha20Poly1305,
    secure : bool,
}

impl CryptoEngine {
    pub fn new() -> Self {
        let key = [0u8; 32]; // Replace with a secure random key in production
        let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));
        Self { cipher, secure : false}
    }
    pub fn set_key (&mut self, key: &[u8; 32]) {
        self.cipher = XChaCha20Poly1305::new(Key::from_slice(key));
        self.secure = true;
    }
    pub fn get_secure(&self) -> bool {
        self.secure
    }
    pub fn new_from_key(key: &[u8; 32]) -> Self {
        let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
        Self { cipher, secure : true }
    }
    pub fn encrypt(&self, plaintext: &[u8]) -> Result< Vec<u8>, Box<dyn Error>> {
        if !self.secure {
            return Err("CryptoEngine not initialized with a secure key".into());
        }
        let rand = rand::random::<[u8; 24]>();
        let nonce = XNonce::from_slice(&rand);
        let mut ciphertext = self.cipher.encrypt(nonce, plaintext).unwrap();
        let mut output = nonce.to_vec();
        output.append(&mut ciphertext);
        Ok(output)
    }
    pub fn decrypt(&self, data: &[u8]) -> Result< Vec<u8>, Box<dyn Error>> {
        if !self.secure {
            return Err("CryptoEngine not initialized with a secure key".into());
        }
        if data.len() < 24 {
            return Err("Data too short to contain nonce".into());
        }
        let (nonce, ciphertext) = data.split_at(24);
        Ok(self.cipher.decrypt(XNonce::from_slice(nonce), ciphertext).unwrap())
    }
}
