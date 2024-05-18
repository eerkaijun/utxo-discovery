use crate::types::Transaction;
use crate::utils::matrix_multiply;

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
    db: Vec<Vec<f64>>,
    // subset of database
    pub a: Vec<Vec<f64>>,
    // hint database
    pub h: Vec<Vec<f64>>
}

impl Server {
    pub fn new() -> Self {
        let matrix_size = 8;
        let lwe_size = 4;
        let mut matrix = vec![vec![0 as f64; matrix_size]; matrix_size];
        let mut submatrix = vec![vec![0 as f64; lwe_size]; matrix_size];

        // populate matrix with 0-63
        let mut value = 0;
        for i in 0..matrix_size {
            for j in 0..matrix_size {
                matrix[i][j] = value as f64;
                value += 1;
            }
        }

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
            q: 23,
            p: 7,
            db: matrix,
            a: submatrix,
            h: hint
        }
    }

    pub fn process_query(self) {
        // Take in query argument and perform PIR, return encrypted data
        println!("Database vector: {:?}", self.db);

        println!("A vector: {:?}", self.a);

        println!("H vector: {:?}", self.h);
    }
}