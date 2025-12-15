use pqc_kyber::{
    keypair, encapsulate, decapsulate,
    KYBER_PUBLICKEYBYTES, KYBER_SECRETKEYBYTES, KYBER_CIPHERTEXTBYTES, KYBER_SSBYTES
};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use std::error::Error;
use std::fmt;

// Type aliases for easier usage
pub type KyberPubKey = [u8; KYBER_PUBLICKEYBYTES];
pub type KyberSecKey = [u8; KYBER_SECRETKEYBYTES];
pub type KyberCipher = [u8; KYBER_CIPHERTEXTBYTES];
pub type SharedSecret = [u8; KYBER_SSBYTES];

#[derive(Debug)]
pub struct CryptoError(String);

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CryptoError: {}", self.0)
    }
}

impl Error for CryptoError {}

pub struct CryptoManager;

impl CryptoManager {
    /// Generates a Kyber-768 keypair.
    pub fn generate_kyber_keypair() -> Result<(KyberPubKey, KyberSecKey), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        match keypair(&mut rng) {
            Ok(keys) => Ok((keys.public, keys.secret)),
            Err(e) => Err(Box::new(CryptoError(format!("{:?}", e))))
        }
    }

    /// Encapsulates a shared secret using the recipient's public key.
    /// Returns (SharedSecret, Ciphertext).
    pub fn encapsulate_secret(pk: &KyberPubKey) -> Result<(SharedSecret, KyberCipher), Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        match encapsulate(pk, &mut rng) {
            Ok((cipher, shared)) => Ok((shared, cipher)),
            Err(e) => Err(Box::new(CryptoError(format!("{:?}", e))))
        }
    }

    /// Decapsulates the shared secret using the recipient's secret key.
    pub fn decapsulate_secret(ct: &KyberCipher, sk: &KyberSecKey) -> Result<SharedSecret, Box<dyn Error>> {
        match decapsulate(ct, sk) {
            Ok(shared) => Ok(shared),
            Err(e) => Err(Box::new(CryptoError(format!("{:?}", e))))
        }
    }

    /// Encrypts a payload using AES-256-GCM and the Shared Secret.
    /// The SharedSecret (32 bytes) is used directly as the AES Key.
    /// Returns (Nonce, Ciphertext).
    pub fn encrypt_aes(secret: &SharedSecret, plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let key = Key::<Aes256Gcm>::from_slice(secret);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(&nonce, plaintext).map_err(|e| CryptoError(format!("Encryption failure: {}", e)))?;

        Ok((nonce.to_vec(), ciphertext))
    }

    /// Decrypts a payload using AES-256-GCM.
    pub fn decrypt_aes(secret: &SharedSecret, nonce_bytes: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let key = Key::<Aes256Gcm>::from_slice(secret);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| CryptoError(format!("Decryption failure: {}", e)))?;

        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kyber_handshake() {
        let (pk, sk) = CryptoManager::generate_kyber_keypair().unwrap();

        // Bob encapsulates
        let (shared_bob, cipher) = CryptoManager::encapsulate_secret(&pk).unwrap();

        // Alice decapsulates
        let shared_alice = CryptoManager::decapsulate_secret(&cipher, &sk).unwrap();

        assert_eq!(shared_bob, shared_alice);
    }

    #[test]
    fn test_aes_encryption_cycle() {
        let secret = [42u8; 32]; // Mock 32-byte secret
        let message = b"Hello Quantum World";

        let (nonce, ciphertext) = CryptoManager::encrypt_aes(&secret, message).unwrap();
        let decrypted = CryptoManager::decrypt_aes(&secret, &nonce, &ciphertext).unwrap();

        assert_eq!(message.to_vec(), decrypted);
    }
}
