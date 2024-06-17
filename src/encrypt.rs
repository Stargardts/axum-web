use aes_gcm::aead::{Aead, KeyInit, OsRng, generic_array::GenericArray};
use aes_gcm::Aes256Gcm/* , Nonce */; // Or `Aes128Gcm`
use rand::RngCore;
use hex;

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    key
}

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn encrypt(key: &[u8], plaintext: &str) -> (String, String) {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = generate_nonce();
    let ciphertext = cipher.encrypt(GenericArray::from_slice(&nonce), plaintext.as_bytes())
        .expect("encryption failure!");

    (hex::encode(nonce), hex::encode(ciphertext))
}

// pub fn decrypt(key: &[u8], nonce: &str, ciphertext: &str) -> String {
//     let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
//     let nonce = hex::decode(nonce).expect("invalid nonce");
//     let ciphertext = hex::decode(ciphertext).expect("invalid ciphertext");
//
//     let plaintext = cipher.decrypt(GenericArray::from_slice(&nonce), ciphertext.as_ref())
//         .expect("decryption failure!");
//
//     String::from_utf8(plaintext).expect("invalid utf-8")
// }
