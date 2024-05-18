pub mod protocol;
pub mod server;
pub mod types;
pub mod utils;

use crate::protocol::UtxoProtocol;
use crate::server::Server;
use crate::utils::{matrix_add, matrix_multiply, transpose};

fn main() {
    println!("Starting UTXO note discovery protocol...");

    // Instantiate server
    let server = Server::new();

    // Instantiate protocol
    // let protocol = UtxoProtocol::new(server);

    let a_vector = server.a.clone();
    let s_vector = vec![vec![1 as f64; 1]; server.n as usize];

    let a_s = transpose(&matrix_multiply(&a_vector, &s_vector));
    
    let error_matrix = vec![0 as f64; server.m as usize];
    let coefficient = server.q as f64 / server.p as f64;
    let column_matrix = vec![coefficient,0.0,0.0,0.0,0.0,0.0,0.0,0.0];

    let query = matrix_add(&matrix_add(&a_s, &error_matrix), &column_matrix);


    server.process_query();
}
