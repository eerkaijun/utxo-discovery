pub mod encryption;
pub mod protocol;
mod rsa;
pub mod server;
pub mod types;
pub mod utils;

use kv::Raw;

use crate::encryption::data_encryption::{encrypt_data_with_public_key, hash_private_key};
use crate::encryption::db::EncryptedDb;
use crate::protocol::UtxoProtocol;
use crate::rsa::{gen_priv_key, gen_pub_key_from_priv_key};
use crate::server::Server;

fn main() {
    println!("Starting UTXO note discovery protocol...");
    let priv_key_bit_size = 2048;
    let alice_private_key = gen_priv_key(priv_key_bit_size);
    let alice_pub_key = gen_pub_key_from_priv_key(&alice_private_key);

    let bob_private_key = gen_priv_key(priv_key_bit_size);
    let bob_pub_key = gen_pub_key_from_priv_key(&bob_private_key);

    // Receiver Shared secret db
    let receiver_shared_secret_db = EncryptedDb::new("./receiver_shared_secret_db");

    // Sender Shared secret db
    let sender_shared_secret_db = EncryptedDb::new("./sender_shared_secret_db");

    // Instantiate server
    let server = Server::new(8, 4, 23, 7);

    // Instantiate protocol
    let protocol = UtxoProtocol::new(server);

    // Retrieve column index 0
    protocol.generate_query(0);

    // TODO: generate shared_secret instead of using hardcoded data
    let x: usize = 12343534234;
    let x_bytes = x.to_be_bytes();

    // Receiver
    // Bob (receiver) to hash and encrypt is data and will send to the server hosting the db
    let bob_hash_priv_key = hash_private_key(&bob_private_key);
    let bob_encrypt_x = encrypt_data_with_public_key(&bob_pub_key, &x_bytes);

    // Receiver shared secret db store encrypted data
    receiver_shared_secret_db.store(&Raw::from(bob_hash_priv_key), &Raw::from(bob_encrypt_x))

    
}

/*
Alice (sender), Bob (receiver)

Storing phase
1. Alice and Bob created shared secret key, x
2. Receiver- encrypt x using Bob's public key
           - hash (receiver private key)
           - store to receiver shared secret db as (key: hash(private_key), value: encrypt(x))
3. Sender - hash (sender private key)
          - try to get value from sender shared secret db using key: hash(sender_private_key)
          - if has value,
                - decode using sender_private_key -> convert to hashmap
          - else if no value,
                - init a haspmap
          - add map(receiver) -> x to hash map
          - convert hash map back to json bytes
          - encrypt(json_bytes) using sender's public key
          - store to sender shared secret db as (key: hash(private_key), value:encrypt(json_bytes))
4. Create a note representing transfer from Alic to Bob
    Note structure : [Cryptographic tag: H(x|i), data: transfer value]
5. store note to Oblivious transfer DB


Retrieving phase
1. Bob as a receiver, should be able to retrieve shared secret key,x between Alice and Bob from receiver
   shared secret db
2. Bob will query OT db for all tags H(x| nonce_i). where nonce_i is from 0 to a big number (hardcoded by us)
3. Bob will receive all the required data from OT db
4. Bob is able to decrypt the data and obtain its balance
*/
