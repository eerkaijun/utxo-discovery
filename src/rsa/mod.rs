use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub fn gen_priv_key(bit_size: usize) -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bit_size);
    priv_key.unwrap()
}

pub fn gen_pub_key_from_priv_key(private_key: &RsaPrivateKey) -> RsaPublicKey {
    RsaPublicKey::from(private_key)
}

pub fn encrypt_data(pub_key: RsaPublicKey, data: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    enc_data
}

pub fn decrypt_data(priv_key: RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, ciphertext)
        .expect("failed to decrypt");
    dec_data
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rsa::pkcs1::EncodeRsaPublicKey;

    use crate::encryption::{
        data_encryption::encrypt_data_with_public_key, sender_pov_db::hash_map_to_json_bytes,
    };

    use super::{gen_priv_key, gen_pub_key_from_priv_key};

    #[test]
    fn test_encrypt_json_bytes() {
        let priv_key = gen_priv_key(2048);
        let pub_key = gen_pub_key_from_priv_key(&priv_key);

        let mut map: HashMap<String, usize> = HashMap::new();
        let der = pub_key.to_pkcs1_der().expect("Failed to encode DER");
        let hex_string = hex::encode(der);

        map.insert(String::from("1234"), 1234);
        // map.insert(hex_string, 1234);
        let json_bytes = hash_map_to_json_bytes(map);

        // let json_bytes = br#"{ "12345": 123, "22345": 355, "9762342343": 796987687 }"#;

        let encrypted_data = encrypt_data_with_public_key(&pub_key, &json_bytes);
    }
}
