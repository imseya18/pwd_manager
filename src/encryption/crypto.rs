use ring::{pbkdf2, rand::{self, SecureRandom}, digest};
use std::num::NonZeroU32;

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
            salt, // Assurez-vous que le sel est unique pour chaque clé générée
            password.as_bytes(),
            &mut key,
        );
        //println!("Clé AES dérivée: {:?} avec {:?}", key, salt);
        key
    }
}

