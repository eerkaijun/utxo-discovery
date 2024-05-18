use std::collections::HashMap;

// UTXO transaction object when sending a private transfer
// Usually include a blinding field but omitted for simplicity here
pub struct Transaction {
    pub_key: String,
    amount: u128
}

impl Transaction {
    pub fn new(pub_key: String, amount: u128) -> Self {
        Self {
            pub_key,
            amount
        }
    }
}

// Information that a user publishes when first entering the protocol
// The valid set will be between H(x | random_nonce) to H(x | random_nonce + threshold_constant)
// where H is a hash function and x is a shared secret between the user and the sender
// When all the indices between the range has been used by a sender, the user have to renew its public info
pub struct PublicUserInfo {
    random_nonce: u128,
    threshold_constant: u32
}

impl PublicUserInfo {
    pub fn new(random_nonce: u128, threshold_constant: u32) -> Self {
        Self {
            random_nonce,
            threshold_constant
        }
    }
}

struct UtxoProtocol {
    // maps a user public key to its public information
    user_info: HashMap<String, PublicUserInfo>,
    // TODO: server instance
}

impl UtxoProtocol {
    pub fn new() -> Self {
        Self {
            user_info: HashMap::new()
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