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
    // let data = b"hello world";
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
