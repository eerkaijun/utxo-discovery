use std::time::Instant;
use tfhe::{set_server_key, FheUint8, ServerKey};

use crate::types::Transaction;

pub struct Server {
    // tuple of [H(x|tag), Transaction]
    published_tags: Vec<(String, Transaction)>
}

impl Server {
    pub fn new() -> Self {
        Self {
            published_tags: vec![]
        }
    }

    pub fn get_published_tags(self) -> Vec<(String, Transaction)> {
        self.published_tags
    }

    pub fn process_query(self, query: FheUint8, data: FheUint8, server_key: ServerKey) -> FheUint8 {
        // Take in query argument and perform PIR, return encrypted data
        let start_time = Instant::now();
        set_server_key(server_key);
        let query_response = query * data;
        let end_time = Instant::now();
        println!("Time taken on server side: {:?}", end_time - start_time);
        
        return query_response;
    }
}