use bytes::{BufMut, BytesMut};
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::{messages, utils, MAGIC_BYTES};
use messages::BitcoinMessage;

pub async fn send_message(
    stream: &mut TcpStream,
    command: &str,
    payload: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    let checksum = utils::double_sha256(&payload)[..4].to_vec();
    let mut message = BytesMut::with_capacity(24 + payload.len());

    message.put_slice(MAGIC_BYTES); // Magic bytes
    message.put_slice(command.as_bytes()); // Command name padded to 12 bytes
    message.put_u32_le(payload.len() as u32); // Payload length
    message.put_slice(&checksum); // Checksum
    message.put_slice(&payload); // Payload

    stream
        .write_all(&message)
        .await
        .expect("Error writing message");

    println!("Sent '{}' message", command);

    Ok(())
}

pub async fn read_message(
    stream: &mut TcpStream,
) -> Result<messages::BitcoinMessage, Box<dyn Error>> {
    // First read the Bitcoin message header - which is exactly 24 bytes
    let mut header_buffer = [0u8; 24];
    if let Err(e) = utils::read_exact_bytes(stream, &mut header_buffer).await {
        if e.to_string().contains("Stream might have been closed") {
            // Stream might have been closed, but connection is still alive
            // TODO: Reconsider this approach
            return Ok(BitcoinMessage::Empty);
        } else {
            return Err(e);
        }
    }

    // Verify magic bytes
    if &header_buffer[..4] != MAGIC_BYTES {
        return Err("Invalid magic bytes".into());
    }

    // Extract command
    let command = String::from_utf8_lossy(&header_buffer[4..16])
        .trim_end_matches('\0')
        .to_string();

    // Read the payload length
    let payload_length = u32::from_le_bytes(
        header_buffer[16..20]
            .try_into()
            .expect("Error converting payload length"),
    );

    // Read the payload
    let mut payload_buffer = vec![0u8; payload_length as usize];
    stream
        .read_exact(&mut payload_buffer)
        .await
        .expect("Error reading payload");

    Ok(parse_message(&command, &payload_buffer))
}

pub fn parse_message(command: &str, payload: &[u8]) -> BitcoinMessage {
    match command {
        "version" => {
            messages::parse_version_payload(&payload).expect("Error parsing version message");
            BitcoinMessage::Version(payload.to_vec())
        }
        "verack" => BitcoinMessage::VerAck,
        "ping" => {
            let nonce = u64::from_le_bytes(payload.try_into().unwrap());
            BitcoinMessage::Ping(nonce)
        }
        "pong" => {
            let nonce = u64::from_le_bytes(payload.try_into().unwrap());
            BitcoinMessage::Pong(nonce)
        }
        _ => BitcoinMessage::Other(command.to_string(), payload.to_vec()),
    }
}

pub async fn handle_message(
    stream: &mut TcpStream,
    message: BitcoinMessage,
) -> Result<bool, Box<dyn Error>> {
    match message {
        BitcoinMessage::Version(_payload) => {
            println!("Received 'version' message");
            send_message(
                stream,
                "verack",
                Vec::new(), // Empty payload
            )
            .await
            .expect("Error sending verack");
        }
        BitcoinMessage::VerAck => {
            println!("Received 'verack' message");

            return Ok(true); // Handshake complete
        }
        BitcoinMessage::Ping(nonce) => {
            println!("Received 'ping' message with nonce: {}", nonce);

            send_message(stream, "pong", nonce.to_le_bytes().to_vec())
                .await
                .expect("Error sending pong");
        }
        BitcoinMessage::Pong(nonce) => {
            println!("Received 'pong' message with nonce: {}", nonce);
        }
        BitcoinMessage::Other(command, payload) => {
            println!("Received '{}' message: {:?}", command, payload);
        }
        BitcoinMessage::Empty => return Ok(false),
    }
    Ok(false)
}
