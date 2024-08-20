/*use pbkdf2::{pbkdf2, Params};
use hmac::{Hmac};
use sha2::Sha256;
use rand::{thread_rng, RngCore, distributions::Alphanumeric};
*/
#[derive(Debug)]
pub struct Crypto {

}

impl Crypto {

/*
    pub fn create_key_from_password(password: &str) -> [u8; 32]
    {
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt);

        
        let iterations = 100_000; // Nombre d'itérations élevé pour plus de sécurité
        let kdf_params = Params {
            rounds: iterations,
            output_length: 32, // Sortie de 256 bits pour AES-256
        };

        // Dérivation de la clé
        let mut key = [0u8; 32];
        let password_bytes: &[u8] = password.as_bytes();
        pbkdf2::<Hmac<Sha256>>(password_bytes, &salt, kdf_params, &mut key);

        // La clé `key` peut maintenant être utilisée pour le chiffrement AES
        println!("Clé AES dérivée: {:?} avec {:?}", key, salt);
        key
    } */
}

