use crate::types::Transaction;
use crate::utils::{detranspose, matrix_multiply, transpose};

pub struct Server {
    // tuple of [H(x|tag), Transaction]
    published_tags: Vec<(String, Transaction)>,

    /// private information retrieval parameters, for notation refer to 
    /// https://forum.aztec.network/t/note-discovery-proposal-rfp/3247#hintlesspir-7
    
    // database dimension
    pub m: u128,
    // lwe dimension
    pub n: u128,
    // ciphertext modulus
    pub q: u128,
    // plaintext modules
    pub p: u128,

    // database of size m rows and m columns
    pub db: Vec<Vec<f64>>,
    // subset of database of size m rows and n columns
    pub a: Vec<Vec<f64>>,
    // hint database of size m rows and n columns
    pub h: Vec<Vec<f64>>
}

impl Server {
    pub fn new(
        matrix_size: usize,
        lwe_size: usize,
        ciphertext_modulus: u128,
        plaintext_modulus: u128,
    ) -> Self {
        let matrix = vec![vec![0 as f64; matrix_size]; matrix_size];
        let mut submatrix = vec![vec![0 as f64; lwe_size]; matrix_size];

        // submatrix is a randomly sampled column subset of matrix
        for i in 0..matrix_size {
            for j in 0..lwe_size {
                submatrix[i][j] = matrix[i][j];
            }
        }

        // hint matrix is the matmul of matrix and submatrix
        let hint = matrix_multiply(&matrix, &submatrix);

        Self {
            published_tags: vec![],
            m: matrix_size as u128,
            n: lwe_size as u128,
            q: ciphertext_modulus,
            p: plaintext_modulus,
            db: matrix,
            a: submatrix,
            h: hint
        }
    }

    // A and H matrix needs to be updated when db is updated
    fn update(&mut self) {
        let matrix = self.db.clone();

        // submatrix is a randomly sampled column subset of matrix
        let mut submatrix = vec![vec![0 as f64; self.n as usize]; self.m as usize];
        for i in 0..self.m as usize {
            for j in 0..self.n as usize {
                submatrix[i][j] = matrix[i][j];
            }
        }

        // hint matrix is the matmul of matrix and submatrix
        let hint = matrix_multiply(&matrix, &submatrix);

        self.a = submatrix;
        self.h = hint;
    }

    pub fn publish_to_database(
        &mut self,
        row_index: usize,
        column_index: usize,
        item: f64
    ) {
        self.db[row_index][column_index] = item;
        self.update();
    }

    pub fn process_query(&self, query: &Vec<f64>) -> Vec<f64> {
        // Take in query argument and perform PIR, return encrypted data
        // Db * q
        detranspose(&matrix_multiply(&self.db, &transpose(query)))
    }
}