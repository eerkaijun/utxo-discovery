use crate::types::Transaction;

pub struct Server {
    // tuple of [H(x|tag), Transaction]
    published_tags: Vec<(String, Transaction)>,

    /// private information retrieval parameters, for notation refer to 
    /// https://forum.aztec.network/t/note-discovery-proposal-rfp/3247#hintlesspir-7
    
    // database dimension
    m: u128,
    // lwe dimension
    n: u128,
    // ciphertext modulus
    q: u128,
    // plaintext modules
    p: u128,

    // database of size m rows and m columns
    matrix: Vec<Vec<u128>>,
}

impl Server {
    pub fn new() -> Self {
        let matrix_size = 8;
        let matrix = vec![vec![0; matrix_size]; matrix_size];
        Self {
            published_tags: vec![],
            m: matrix_size as u128,
            n: 4,
            q: 23,
            p: 7,
            matrix
        }
    }

    pub fn process_query() {
        // Take in query argument and perform PIR, return encrypted data
    }
}