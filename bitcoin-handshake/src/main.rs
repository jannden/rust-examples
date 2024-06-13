use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

mod communication;
mod messages;
mod utils;

pub const BITCOIN_NODE: &str = "45.79.127.28:8333"; // Make sure it's a working Bitcoin node address
pub const MAGIC_BYTES: &[u8] = b"\xf9\xbe\xb4\xd9"; // Mainnet
pub const USER_AGENT: &str = "my-app-name"; // Whatever name you want
pub const PROTOCOL_VERSION: u32 = 70015;

use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(BITCOIN_NODE)
        .await
        .expect("Failed to connect to Bitcoin node");
    println!("Connected to Bitcoin node at {}", BITCOIN_NODE);

    let version_message = messages::create_version_payload();
    stream
        .write_all(&version_message)
        .await
        .expect("Error writing version message");

    let mut handshake_complete = false;

    while !handshake_complete {
        let message = communication::read_message(&mut stream).await?;
        handshake_complete = communication::handle_message(&mut stream, message).await?;
    }

    println!("Handshake complete.");

    communication::send_message(
        &mut stream,
        "getblocks",
        messages::create_getblocks_payload(),
    )
    .await
    .expect("Error sending getblocks");

    let mut ping_interval = interval(Duration::from_secs(10));
    loop {
        tokio::select! {
            message = communication::read_message(&mut stream) => {
                let message = message?;
                communication::handle_message(&mut stream, message).await?;
            }

            _ = ping_interval.tick() => {

              communication::send_message(&mut stream, "ping", rand::random::<u64>().to_le_bytes().to_vec())
                .await
                .expect("Error sending ping");
          }
        }
    }
}
