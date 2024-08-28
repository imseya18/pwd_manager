use ring::{pbkdf2, rand::{self, SecureRandom}, digest};
use std::num::NonZeroU32;
use aes::Aes256;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::ChaCha20Poly1305;

use serde::{Serialize, Deserialize};
use serde_json;
use aes::{BlockEncrypt, NewBlockCipher};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;
use ring::error::Unspecified;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN; // Longueur de la clé AES 256 bits
const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) }; // Nombre d'itérations recommandé pour PBKDF2

#[derive(Debug)]
pub struct Crypto {

}

impl Crypto {

    pub fn generate_rnd_salt() -> Result<[u8; 16]> {
        let mut salt = [0u8; 16];
        ring::rand::SystemRandom::new()
            .fill(&mut salt)
            .map_err(|e: Unspecified| Error::from(format!("Failed to generate salt: {:?}", e)))?;
        Ok(salt)
    }

    pub fn create_key_from_password(password: &str, salt: &[u8; 16]) -> [u8; CREDENTIAL_LEN] {
        let mut key = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            ITERATIONS,
            salt,
            password.as_bytes(),
            &mut key,
        );
        key
    }

    pub fn encrypt(cleartext: &str, key: &[u8]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::from("Key must be 32 bytes long"));
        }
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut obsf = cipher.encrypt(&nonce, cleartext.as_bytes()).map_err(|e| Error::from(format!("Encryption failed: {}", e)))?;
        obsf.splice(..0, nonce.iter().copied());
        Ok(obsf)
    }

    pub fn decrypt(obsf: &[u8], key: &[u8]) -> Result<String> {
        if key.len() != 32 {
            return Err(Error::from("Key must be 32 bytes long"));
        }
        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
        let (nonce, ciphertext) = obsf.split_at(NonceSize::to_usize());
        let nonce = GenericArray::from_slice(nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| Error::from(format!("Decryption failed: {}", e)))?;
        Ok(String::from_utf8(plaintext)?)
    }

    pub fn encrypt_for_storage(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::from("Key must be 32 bytes long"));
        }
        let iv: [u8; 16] = Crypto::generate_rnd_salt()?;
        let cipher = Aes256Cbc::new_from_slices(key, &iv)?;
        let mut encrypted = iv.to_vec();
        encrypted.extend_from_slice(&cipher.encrypt_vec(data));
        Ok(encrypted)
      }

    pub fn decrypt_from_storage(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>> {
        if key.len() != 32 {
            return Err(Error::from("Key must be 32 bytes long"));
        }
        let (iv, ciphertext) = data.split_at(16);
        let cipher = Aes256Cbc::new_from_slices(key, iv)?;
        Ok(cipher.decrypt_vec(ciphertext)?)
      }
}
