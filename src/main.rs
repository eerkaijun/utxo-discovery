pub mod protocol;
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

    protocol.generate_query();
}
