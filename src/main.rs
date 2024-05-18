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
