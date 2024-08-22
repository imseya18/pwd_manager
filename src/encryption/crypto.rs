use ring::{pbkdf2, rand::{self, SecureRandom}, digest};
use std::num::NonZeroU32;
use aes::Aes256;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, OsRng};
use chacha20poly1305::ChaCha20Poly1305;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN; // Longueur de la clé AES 256 bits
const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) }; // Nombre d'itérations recommandé pour PBKDF2

#[derive(Debug)]
pub struct Crypto {

}

impl Crypto {

    pub fn generate_rnd_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];

        rand::SystemRandom::new().fill(&mut salt).unwrap();
        salt
    }

    pub fn create_key_from_password(password: &str, salt: &[u8]) -> [u8; CREDENTIAL_LEN] {
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

    pub fn encrypt(cleartext: &str, key: &[u8]) -> Vec<u8> {
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let mut obsf = cipher.encrypt(&nonce, cleartext.as_bytes()).unwrap();
        obsf.splice(..0, nonce.iter().copied());
        obsf
    }

    pub fn decrypt(obsf: &[u8], key: &[u8]) -> String {
        type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
        let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
        let (nonce, ciphertext) = obsf.split_at(NonceSize::to_usize());
        let nonce = GenericArray::from_slice(nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext).unwrap();
        String::from_utf8(plaintext).unwrap()
    }
}

