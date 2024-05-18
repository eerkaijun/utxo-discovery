use std::collections::HashMap;
use crate::server::Server;
use crate::types::PublicUserInfo;
use crate::utils::{detranspose, matrix_add, matrix_multiply, matrix_sub};

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
        // public matrices by server
        let a_matrix = self.server_instance.a.clone();
        let h_matrix = self.server_instance.h.clone();

        // query specific secret vector
        // TODO: secret vector should be generate randomly
        let s_vector = vec![vec![1 as f64; 1]; self.server_instance.n as usize];

        // A * s
        let a_s = detranspose(&matrix_multiply(&a_matrix, &s_vector));
    
        // TODO: should be set as a gaussian distribution with standard deviation of LWE
        let error_matrix = vec![0 as f64; self.server_instance.m as usize];

        // Q / P
        let scale_factor = self.server_instance.q as f64 / self.server_instance.p as f64;

        // (Q / P) * col_i 
        let column_matrix = vec![scale_factor, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        // q = A * s + E + (Q/P) * col_i
        let query = matrix_add(&matrix_add(&a_s, &error_matrix), &column_matrix);

        // server response is Db * q
        let response = self.server_instance.process_query(&query);

        // H * s
        let h_s = detranspose(&matrix_multiply(&h_matrix.clone(), &s_vector));

        // Db * q - H * s
        let scaled_result = matrix_sub(&response, &h_s);

        // scale down by Q / P
        let mut retrieved_data = vec![0.0; 8];
        for i in 0..8 {
            retrieved_data[i] = scaled_result[i] / scale_factor;
        }

        println!("Retrieved data: {:?}", retrieved_data);
    }
}