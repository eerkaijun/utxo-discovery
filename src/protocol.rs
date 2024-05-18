use std::collections::HashMap;
use crate::server::Server;
use crate::types::{PublicUserInfo, Transaction};

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

    pub fn generate_query() {
        // Helper function to generate query to be sent to the server

    }
}