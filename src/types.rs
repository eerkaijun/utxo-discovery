// UTXO transaction object when sending a private transfer
// Usually include a blinding field but omitted for simplicity here
pub struct Transaction {
    pub_key: u32,
    amount: f32
}

impl Transaction {
    pub fn new(pub_key: u32, amount: f32) -> Self {
        Self {
            pub_key,
            amount
        }
    }

    // Function to serialize Transaction to f64
    pub fn serialize_to_f64(&self) -> f64 {
        // Convert the u32 pub_key to a u64 by shifting it to the upper 32 bits
        let pub_key_u64 = (self.pub_key as u64) << 32;

        // Convert the f32 amount to a u32 representation
        let amount_u32 = self.amount.to_bits();

        // Combine the pub_key and amount into a single u64
        let combined = pub_key_u64 | amount_u32 as u64;

        // Convert the combined u64 to f64
        combined as f64
    }

    // Function to deserialize f64 back to Transaction
    pub fn deserialize_from_f64(value: f64) -> Self {
        let combined = value as u64;

        // Extract the pub_key by shifting right 32 bits
        let pub_key = (combined >> 32) as u32;

        // Extract the amount by masking the lower 32 bits and converting back to f32
        let amount_bits = combined as u32;
        let amount = f32::from_bits(amount_bits);

        Self { pub_key, amount }
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