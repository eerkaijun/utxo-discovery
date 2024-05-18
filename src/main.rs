pub mod encryption;
pub mod protocol;
mod rsa;
pub mod server;
pub mod types;
pub mod utils;

use crate::protocol::UtxoProtocol;
use crate::server::Server;

fn main() {
    println!("Starting UTXO note discovery protocol...");

    // Instantiate server
    let server = Server::new(8, 4, 23, 7);

    // Instantiate protocol
    let protocol = UtxoProtocol::new(server);

    // Retrieve column index 0
    protocol.generate_query(0);
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
