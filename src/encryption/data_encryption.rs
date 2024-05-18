use rsa::{pkcs1::EncodeRsaPrivateKey, RsaPrivateKey, RsaPublicKey};
use sha256::digest as Sha256Digest;

use crate::rsa::{decrypt_data, encrypt_data};

pub fn hash_bytes_sha256(preimage: &[u8]) -> String {
    let hashed_val = Sha256Digest(preimage);
    hashed_val
}

// hash private key
pub fn hash_private_key(priv_key: &RsaPrivateKey) -> Vec<u8> {
    let priv_key_sec_doc = priv_key.to_pkcs1_der().unwrap();
    let priv_key_bytes = priv_key_sec_doc.as_bytes();
    let hashed_key = hash_bytes_sha256(priv_key_bytes);
    hashed_key.into_bytes()
}

pub fn encrypt_data_with_public_key(pub_key: &RsaPublicKey, data: &[u8]) -> Vec<u8> {
    let encrypted_data = encrypt_data(pub_key.clone(), data);
    encrypted_data
}

pub fn decrypt_data_with_priv_key(priv_key: &RsaPrivateKey, encrypted_data: Vec<u8>) -> Vec<u8> {
    let decrypted_data = decrypt_data(priv_key.clone(), &encrypted_data);
    decrypted_data
}


// #[cfg(test)]
// mod tests {
//     #[test] 
//     fn test_encrypt_decrypt() {
//         let priv_key = RS
//     }
// }
