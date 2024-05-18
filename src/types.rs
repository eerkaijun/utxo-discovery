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