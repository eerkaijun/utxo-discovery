pub mod encryption;
pub mod protocol;
mod rsa;
pub mod server;
pub mod types;

use crate::protocol::UtxoProtocol;
use crate::server::Server;

fn main() {
    println!("Starting UTXO note discovery protocol...");

    // Instantiate server
    let server = Server::new();

    // Instantiate protocol
    let protocol = UtxoProtocol::new(server);
}
