use crate::types::{PublicUserInfo, Transaction};

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

    pub fn process_query() {
        // Take in query argument and perform PIR, return encrypted data
    }
}