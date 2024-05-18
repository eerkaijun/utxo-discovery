pub mod db;
pub mod data_encryption;

use kv::{Bucket, Config, Raw, Store};
use rsa::{pkcs1::EncodeRsaPrivateKey, RsaPrivateKey};
use sha256::digest as Sha256Digest;

use crate::rsa::{encrypt_data, gen_pub_key_from_priv_key};

// pub fn init_new_store(file_path: &str) -> Store {
//     let cfg = Config::new(file_path);
//     Store::new(cfg).unwrap()
// }

// pub fn open_bucket(store: Store, bucket_name: &str) -> Bucket<Raw, Raw> {
//     let bucket = store.bucket::<Raw, Raw>(Some(bucket_name));
//     bucket.unwrap()
// }

// pub fn store(bucket: Bucket<Raw, Raw>, key: &Raw, value: &Raw) {
//     bucket.set(key, value).unwrap();
// }

// pub fn get(bucket: Bucket<Raw, Raw>, key: &Raw) -> Option<Raw> {
//     let val = bucket.get(key).unwrap();
//     val
// }

// pub struct EncryptedDb<'a> {
//     bucket: Bucket<'a, Raw, Raw>,
// }

// impl<'a> EncryptedDb<'a> {
//     pub fn new(file_path: &str) -> Self {
//         let store = init_new_store(file_path);
//         let bucket = open_bucket(store, "encrypted db");

//         Self { bucket }
//     }

//     //  fn store(bucket: Bucket<Raw, Raw>, key: &Raw, value: &Raw) {
//     //     bucket.set(key, value).unwrap();
//     // }

//     // pub fn get(bucket: Bucket<Raw, Raw>, key: &Raw) -> Option<Raw> {
//     //     let val = bucket.get(key).unwrap();
//     //     val
//     // }

//     fn store(&self, key: &Raw, value: &Raw) {
//         self.bucket.set(key, value).unwrap();
//     }

//     fn hash_bytes_sha256(&self, preimage: &[u8]) -> String {
//         let hashed_val = Sha256Digest(preimage);
//         hashed_val
//     }

//     /*
//      * Outputs Hash(private key)
//      */
//     pub fn hash_priv_key(&self, priv_key: &RsaPrivateKey) -> Vec<u8> {
//         let priv_key_sec_doc = priv_key.to_pkcs1_der().unwrap();
//         let priv_key_bytes = priv_key_sec_doc.as_bytes();
//         let hashed_key = self.hash_bytes_sha256(priv_key_bytes);
//         hashed_key.into_bytes()
//     }

//     /*
//      * This function receives the hash(private key) and the raw data
//      * It will store the hash(private key) as the key of our key value lookup
//      * and it will encrypt raw data with public key and store it in our key value lookup
//      */
//     pub fn encrypt_and_store(&self, priv_key: &RsaPrivateKey, data: &[u8]) -> Option<Raw> {
//         let priv_key_sec_doc = priv_key.to_pkcs1_der().unwrap();
//         let priv_key_bytes = priv_key_sec_doc.as_bytes();
//         let hashed_key = self.hash_bytes_sha256(priv_key_bytes);
//         let hashed_key_bytes = hashed_key.into_bytes();
//         let hashed_key_raw = Raw::from(hashed_key_bytes);

//         let pub_key: rsa::RsaPublicKey = gen_pub_key_from_priv_key(priv_key);
//         let encrypted_data = encrypt_data(pub_key, data);
//         let encrypted_data_raw = Raw::from(encrypted_data);
//         self.bucket
//             .set(&hashed_key_raw, &encrypted_data_raw)
//             .unwrap()
//     }

//     /*
//      * Search from encrypted db by hash private key
//      */
//     pub fn get_and_decrypt(&self) {}
// }
