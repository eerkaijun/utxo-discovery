use rand::Rng;
use rand_distr::{Distribution, Normal};
use sha256::digest as Sha256Digest;
use std::collections::HashMap;
use crate::server::Server;
use crate::types::{PublicUserInfo, Transaction};
use crate::utils::{detranspose, matrix_add, matrix_multiply, matrix_sub};

pub struct UtxoProtocol {
    // maps a user public key to its public information
    user_info: HashMap<u32, PublicUserInfo>,
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

    pub fn register(&mut self, pub_key: u32, random_nonce: u128, threshold_constant: u32) {
        let user_info = PublicUserInfo::new(random_nonce, threshold_constant);
        self.user_info.insert(pub_key, user_info);
    }

    pub fn transfer_and_tag(
        &mut self,
        shared_secret: u128,
        receiver_pub_key: u32,
        amount: f32,
        tag_index: u32
    ) {
        // Generate UTXO Transaction
        let mut tx = Transaction::new(receiver_pub_key, amount).serialize_to_f64() as u64;
        tx %= self.server_instance.q as u64;
        println!("Transaction sent: {:?}", tx);

        // Publish tuple of [H(x|tag), Transaction] to the server
        let concatenated_data = format!("{}{}", shared_secret, tag_index);
        // TODO: convert tag to location in database
        let tag = Sha256Digest(concatenated_data.as_bytes());
        
        self.server_instance.publish_to_database(0, 0, tx as f64);
    }

    // Helper function to generate query to be sent to the server
    pub fn generate_query(&self, column_index: usize) {
        // public matrices by server
        let a_matrix = self.server_instance.a.clone();
        let h_matrix = self.server_instance.h.clone();

        // query specific secret vector (generated randomly through a ternary distribution)
        let s_vector: Vec<Vec<f64>> = (0..self.server_instance.n as usize)
            .map(|_| {
                let random_value = match rand::thread_rng().gen_range(0..=2) {
                    0 => -1.0,
                    1 => 0.0,
                    _ => 1.0,
                };
                vec![random_value as f64; 1]
            })
            .collect();

        // A * s
        let a_s = detranspose(&matrix_multiply(&a_matrix, &s_vector));
    
        // Generate the error matrix with Gaussian distribution with standard deviation of LWE
        let normal_dist = Normal::new(0.0, 0.00001).unwrap();
        let error_matrix: Vec<f64> = (0..self.server_instance.m as usize)
            .map(|_| normal_dist.sample(&mut rand::thread_rng()))
            .collect();

        // Q / P
        let scale_factor = self.server_instance.q as f64 / self.server_instance.p as f64;

        // (Q / P) * col_i 
        let mut column_matrix = vec![0.0; self.server_instance.m as usize];
        column_matrix[column_index] = scale_factor;

        // q = A * s + E + (Q/P) * col_i
        let query = matrix_add(&matrix_add(&a_s, &error_matrix), &column_matrix);

        // server response is Db * q
        let response = self.server_instance.process_query(&query);

        // H * s
        let h_s = detranspose(&matrix_multiply(&h_matrix.clone(), &s_vector));

        // Db * q - H * s
        let scaled_result = matrix_sub(&response, &h_s);

        // scale down by Q / P and round to one decimal
        let mut retrieved_data = vec![0.0; scaled_result.len()];
        for i in 0..scaled_result.len() {
            retrieved_data[i] = (scaled_result[i] / scale_factor * 10.0).round() / 10.0;
        }

        println!("Retrieved data: {:?}", retrieved_data);
    }
}