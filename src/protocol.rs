use std::collections::HashMap;
use tfhe::{ConfigBuilder, generate_keys, FheUint8};
use tfhe::prelude::*;

use crate::server::Server;
use crate::types::PublicUserInfo;

pub struct UtxoProtocol {
    // maps a user public key to its public information
    user_info: HashMap<String, PublicUserInfo>,
    // server instance (single server PIR)
    server_instance: Server,
}

impl UtxoProtocol {
    pub fn new(server_instance: Server) -> Self {
        Self {
            user_info: HashMap::new(),
            server_instance
        }
    }

    pub fn register(mut self, pub_key: String, random_nonce: u128, threshold_constant: u32) {
        let user_info = PublicUserInfo::new(random_nonce, threshold_constant);
        self.user_info.insert(pub_key, user_info);
    }

    pub fn transfer_and_tag(receiver_pub_key: String, amount: u128, tag: u32) {
        // Generate UTXO Transaction

        // Retrieve tag range of receiver

        // Publish tuple of [H(x|tag), Transaction] to the server

    }

    // Helper function to generate query to be sent to the server
    pub fn generate_query(self) {
        // Generate client and server key for private information retrieval
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        // TODO: remove hardcode
        let query = FheUint8::encrypt(5 as u8, &client_key);
        let data = FheUint8::encrypt(20 as u8, &client_key);

        let query_response = self.server_instance.process_query(query, data, server_key);
        let decrypted_response: u8 = query_response.decrypt(&client_key);

        println!("Decrypted response: {:?}", decrypted_response);
    }
}